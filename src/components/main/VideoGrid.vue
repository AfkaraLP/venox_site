<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'
import PlayButton from './PlayButton.vue'

interface VideoEntry {
  id: string
  title: string
  published: string
  [key: string]: any
}

interface ChannelFeed {
  title: string
  link?: string
  channelId?: string
  id?: string
  author?: { name: string, uri: string }
  description?: string
  entry: VideoEntry[]
}

const feeds = ref<ChannelFeed[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

// Track which video is playing per channel
const playingIndexes = ref<{ [channelIdx: number]: number | null }>({})

// Use an object for scrollRefs to avoid array index issues
const scrollRefs = ref<{ [key: number]: HTMLElement | null }>({})
const scrollHandlers = ref<((e: WheelEvent) => void)[]>([])

// Track which section is hovered
const hoveredIndex = ref<number | null>(null)

function setScrollRef(idx: number, el: HTMLElement | null) {
  scrollRefs.value[idx] = el
}

function getYoutubeId(entryId: string) {
  return entryId.split(':').pop() || ''
}
function getThumbnailUrl(entryId: string) {
  const vid = getYoutubeId(entryId)
  return `https://img.youtube.com/vi/${vid}/hqdefault.jpg`
}
function getYoutubeUrl(entryId: string) {
  const vid = getYoutubeId(entryId)
  return `https://www.youtube.com/watch?v=${vid}`
}
function getEmbedUrl(entryId: string) {
  const vid = getYoutubeId(entryId)
  return `https://www.youtube.com/embed/${vid}`
}
function playVideo(channelIdx: number, videoIdx: number) {
  playingIndexes.value[channelIdx] = videoIdx
}

function handleScrollHijack(e: WheelEvent, idx: number) {
  const el = scrollRefs.value[idx]
  if (!el) return
  if (el.scrollWidth > el.clientWidth) {
    // Only hijack if not at the very left or right
    const atStart = el.scrollLeft === 0 && e.deltaY < 0
    const atEnd = Math.abs(el.scrollLeft + el.clientWidth - el.scrollWidth) < 2 && e.deltaY > 0
    if (atStart || atEnd) return // allow normal vertical scroll at edges
    el.scrollLeft += e.deltaY * 1.5 // less aggressive than music section
    e.preventDefault()
  }
}

function attachWheel(idx: number) {
  const el = scrollRefs.value[idx]
  if (el && !scrollHandlers.value[idx]) {
    const handler = (e: WheelEvent) => handleScrollHijack(e, idx)
    scrollHandlers.value[idx] = handler
    el.addEventListener('wheel', handler, { passive: false })
  }
}
function detachWheel(idx: number) {
  const el = scrollRefs.value[idx]
  if (el && scrollHandlers.value[idx]) {
    el.removeEventListener('wheel', scrollHandlers.value[idx])
    scrollHandlers.value[idx] = undefined
  }
}

onMounted(async () => {
  try {
    const res = await fetch('/api/youtube_videos')
    if (!res.ok) throw new Error('Failed to fetch videos')
    const apiFeeds = await res.json()
    feeds.value = apiFeeds.map((feed: any) => ({
      title: feed.title,
      link: feed.link || feed.author?.uri,
      author: feed.author,
      channelId: feed.channelId,
      id: feed.id,
      description: feed.subtitle || feed.description,
      entry: Array.isArray(feed.entry) ? feed.entry : []
    }))
    await nextTick()
    // No global wheel listeners; handled on hover
  } catch (e: any) {
    error.value = e.message || 'Unknown error'
    console.error('VideoGrid fetch error:', e)
  } finally {
    loading.value = false
  }
})

onBeforeUnmount(() => {
  Object.entries(scrollRefs.value).forEach(([idx, el]) => {
    if (el && scrollHandlers.value[Number(idx)]) {
      el.removeEventListener('wheel', scrollHandlers.value[Number(idx)])
    }
  })
})
</script>

<template>
  <section class="video-grid">
    <h2>Latest Videos</h2>
    <div v-if="loading">
      <div class="side-scroll-grid">
        <div class="video-card" v-for="n in 4" :key="'skeleton-' + n">
          <div class="video-embed">
            <div class="skeleton-thumb"></div>
          </div>
          <div class="skeleton-title"></div>
        </div>
      </div>
    </div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else>
      <div v-for="(feed, channelIdx) in feeds" :key="feed.title" class="channel-section">
        <div class="channel-header">
          <h3 class="channel-title">
            <a v-if="feed.channelId" :href="'https://www.youtube.com/channel/UC' + feed.channelId" target="_blank" rel="noopener noreferrer">{{ feed.title }}</a>
            <span v-else>{{ feed.title }}</span>
          </h3>
          <div v-if="feed.description" class="channel-desc">{{ feed.description }}</div>
        </div>
        <div
          class="side-scroll-grid"
          :ref="el => setScrollRef(channelIdx, el as HTMLElement | null)"
          @mouseenter="hoveredIndex = channelIdx; attachWheel(channelIdx)"
          @mouseleave="hoveredIndex = null; detachWheel(channelIdx)"
        >
          <div class="video-card" v-for="(video, idx) in feed.entry" :key="video.id">
            <div class="video-embed">
              <template v-if="playingIndexes[channelIdx] !== idx">
                <img :src="getThumbnailUrl(video.id)" :alt="video.title" />
                <PlayButton :size="54" color="#fff" bgColor="#6aac4b" @click="playVideo(channelIdx, idx)" />
              </template>
              <template v-else>
                <iframe
                  :src="getEmbedUrl(video.id) + '?autoplay=1'"
                  frameborder="0"
                  allowfullscreen
                  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                  title="YouTube video player"
                  class="video-iframe"
                ></iframe>
              </template>
            </div>
            <a :href="getYoutubeUrl(video.id)" class="video-title" target="_blank" rel="noopener noreferrer">{{ video.title }}</a>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.video-grid {
  margin: 2rem 0;
}
.video-grid h2 {
  color: #4e2e13;
  margin-bottom: 1rem;
  text-align: left;
  font-family: var(--font-mc);
  text-shadow: 2px 2px 0 #fff, 0 0 4px #bcbcbc;
  letter-spacing: 1px;
  font-size: 2rem;
}
.channel-section {
  margin-bottom: 2.5rem;
  padding-bottom: 2rem;
  border-bottom: 4px dashed #4e2e13;
}
.channel-header {
  margin-bottom: 1.2rem;
}
.channel-title {
  font-size: 1.35rem;
  color: #4e2e13;
  font-weight: bold;
  margin-bottom: 0.3rem;
  font-family: var(--font-mc);
  text-shadow: 2px 2px 0 #fff, 0 0 4px #bcbcbc;
  text-transform: uppercase;
}
.channel-title a {
  color: #4e2e13;
  text-decoration: none;
  transition: color 0.2s;
}
.channel-title a:hover {
  color: #6aac4b;
  text-decoration: underline;
}
.channel-desc {
  color: #4e2e13;
  font-size: 1.05rem;
  margin-bottom: 0.2rem;
  font-family: var(--font-mc);
}
.side-scroll-grid {
  display: flex;
  flex-direction: row;
  gap: 1.5rem;
  overflow-x: auto;
  padding-bottom: 1rem;
  scroll-behavior: smooth;
  max-width: 100vw;
  min-width: 0;
  scrollbar-width: thin;
  scrollbar-color: #bcbcbc #e3e3e3;
  background: #bcbcbc;
  border: 4px dashed #222;
  box-shadow: 0 8px 0 #4e2e13, 0 12px 0 #bcbcbc;
}
.side-scroll-grid:hover {
  scrollbar-color: #6aac4b #bcbcbc;
}
.side-scroll-grid::-webkit-scrollbar {
  height: 12px;
}
.side-scroll-grid::-webkit-scrollbar-thumb {
  background: #bcbcbc;
  border-radius: 0;
}
.side-scroll-grid:hover::-webkit-scrollbar-thumb {
  background: #6aac4b;
}
.side-scroll-grid::-webkit-scrollbar-track {
  background: #e3e3e3;
  border-radius: 0;
}
.video-card {
  background: #e3e3e3cc;
  border-radius: 0;
  border: 6px double #222;
  box-shadow: 0 8px 0 #4e2e13, 0 12px 0 #bcbcbc;
  overflow: hidden;
  width: 320px;
  max-width: 90vw;
  text-align: center;
  transition: transform 0.15s;
  flex: 0 0 320px;
  font-family: var(--font-mc);
  image-rendering: pixelated;
}
.video-card:hover {
  transform: translateY(-6px) scale(1.03);
  box-shadow: 0 12px 0 #4e2e13, 0 16px 0 #bcbcbc;
}
.video-card img {
  width: 100%;
  display: block;
  image-rendering: pixelated;
}
.video-embed {
  width: 100%;
  aspect-ratio: 16/9;
  margin-bottom: 0.5rem;
  border-radius: 0;
  overflow: hidden;
  box-shadow: 0 4px 0 #4e2e13;
  position: relative;
  background: #000;
  display: block;
  border: 4px dashed #222;
}
.video-iframe {
  width: 100%;
  height: 100%;
  border: none;
  display: block;
  aspect-ratio: 16/9;
  background: #000;
}
.play-btn {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  background: #6aac4b;
  border: 4px double #222;
  border-radius: 0;
  font-size: 2.2rem;
  color: #fff;
  width: 54px;
  height: 54px;
  cursor: pointer;
  box-shadow: 0 4px 0 #4e2e13;
  transition: background 0.2s;
  z-index: 2;
  display: flex;
  align-items: center;
  justify-content: center;
  image-rendering: pixelated;
}
.play-btn:hover {
  background: #4e2e13;
  color: #6aac4b;
}
.video-title {
  display: block;
  padding: 0.5rem 0;
  color: #4e2e13;
  font-weight: bold;
  text-decoration: none;
  transition: color 0.2s;
  text-align: center;
  font-family: var(--font-mc);
  text-shadow: 2px 2px 0 #fff, 0 0 4px #bcbcbc;
  text-transform: uppercase;
  letter-spacing: 1px;
}
.video-title:hover {
  color: #6aac4b;
  text-decoration: underline;
}
.skeleton-thumb {
  width: 100%;
  aspect-ratio: 16/9;
  border-radius: 0;
  background: #bcbcbc;
  border: 4px dashed #222;
  animation: skeleton-loading 1.2s infinite linear;
}
.skeleton-title {
  width: 70%;
  height: 1.3rem;
  border-radius: 0;
  margin: 0.7rem auto 0.5rem auto;
  background: #bcbcbc;
  border: 4px dashed #222;
  animation: skeleton-loading 1.2s infinite linear;
}
@keyframes skeleton-loading {
  0% {
    opacity: 0.7;
  }
  100% {
    opacity: 1;
  }
}
.video-embed img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
  image-rendering: pixelated;
}
</style> 
