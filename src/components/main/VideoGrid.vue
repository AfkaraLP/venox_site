<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'

interface VideoEntry {
  id: string
  title: string
  published: string
  [key: string]: any
}

interface ChannelFeed {
  title: string
  link?: string
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
    <div v-if="loading">Loading videos...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else>
      <div v-for="(feed, channelIdx) in feeds" :key="feed.title" class="channel-section">
        <div class="channel-header">
          <h3 class="channel-title">
            <a v-if="feed.link" :href="feed.link" target="_blank" rel="noopener noreferrer">{{ feed.title }}</a>
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
                <button class="play-btn" @click="playVideo(channelIdx, idx)">▶</button>
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
  color: #5a4fcf;
  margin-bottom: 1rem;
  text-align: left;
}
.channel-section {
  margin-bottom: 2.5rem;
  padding-bottom: 2rem;
  border-bottom: 2px solid #e0c3fc44;
}
.channel-header {
  margin-bottom: 1.2rem;
}
.channel-title {
  font-size: 1.35rem;
  color: #3a2e8c;
  font-weight: bold;
  margin-bottom: 0.3rem;
}
.channel-title a {
  color: #3a2e8c;
  text-decoration: none;
  transition: color 0.2s;
}
.channel-title a:hover {
  color: #5a4fcf;
  text-decoration: underline;
}
.channel-desc {
  color: #7b6ee6;
  font-size: 1.05rem;
  margin-bottom: 0.2rem;
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
  scrollbar-color: #b5aaff44 #f3eaff;
}
.side-scroll-grid:hover {
  scrollbar-color: #7b6ee6 #e0c3fc;
}
.side-scroll-grid::-webkit-scrollbar {
  height: 12px;
}
.side-scroll-grid::-webkit-scrollbar-thumb {
  background: #b5aaff44;
  border-radius: 6px;
}
.side-scroll-grid:hover::-webkit-scrollbar-thumb {
  background: #7b6ee6;
}
.side-scroll-grid::-webkit-scrollbar-track {
  background: #f3eaff;
  border-radius: 6px;
}
.video-card {
  background: #fff;
  border-radius: 1rem;
  box-shadow: 0 2px 12px #b5aaff22;
  overflow: hidden;
  width: 320px;
  max-width: 90vw;
  text-align: center;
  transition: transform 0.15s;
  flex: 0 0 320px;
}
.video-card:hover {
  transform: translateY(-6px) scale(1.03);
}
.video-card img {
  width: 100%;
  display: block;
}
.video-embed {
  width: 100%;
  aspect-ratio: 16/9;
  margin-bottom: 0.5rem;
  border-radius: 1rem;
  overflow: hidden;
  box-shadow: 0 2px 12px #b5aaff22;
  position: relative;
  background: #000;
  display: flex;
  align-items: center;
  justify-content: center;
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
  background: #fff8;
  border: none;
  border-radius: 50%;
  font-size: 2.2rem;
  color: #7b6ee6;
  width: 54px;
  height: 54px;
  cursor: pointer;
  box-shadow: 0 2px 8px #b5aaff44;
  transition: background 0.2s;
  z-index: 2;
  display: flex;
  align-items: center;
  justify-content: center;
}
.play-btn:hover {
  background: #fff;
}
.video-title {
  display: block;
  padding: 0.5rem 0;
  color: #7b6ee6;
  font-weight: bold;
  text-decoration: none;
  transition: color 0.2s;
  text-align: center;
}
.video-title:hover {
  color: #5a4fcf;
  text-decoration: underline;
}
</style> 