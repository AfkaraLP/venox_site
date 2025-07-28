<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'
import PlayButton from './PlayButton.vue'

interface VideoEntry {
  id: string
  title: string
  published: string
  group: Group
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

function timeAgo(utcDate: string | Date): string {
  const now = new Date();
  const date = new Date(utcDate);
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  const intervals: [number, string][] = [
    [60, "second"],
    [60, "minute"],
    [24, "hour"],
    [7, "day"],
    [4.34524, "week"], // approx weeks per month
    [12, "month"],
    [Infinity, "year"],
  ];

  let unitIndex = 0;
  let value = seconds;

  for (const [divisor, name] of intervals) {
    if (value < divisor) {
      const rounded = Math.floor(value);
      return `${rounded} ${name}${rounded !== 1 ? "s" : ""} ago`;
    }
    value /= divisor;
    unitIndex++;
  }

  return "just now"; // fallback
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
                <img :src="video.group.thumbnail.url" :alt="video.title" />
                <PlayButton :size="54" @click="playVideo(channelIdx, idx)" />
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
            <div class="video-stats">
              <span v-if="video.group.community.starRating.count">
                 {{video.group.community.starRating.count}} like<span v-if="video.group.community.starRating.count > 1">s</span>
              </span>
              <span v-if="video.group.community.statistics.views">
                {{video.group.community.statistics.views}} view<span v-if="video.group.community.statistics.views > 1">s</span>
              </span>
              <span v-if="video.published">{{timeAgo(video.published)}}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.video-grid {
  margin: 0 0 2rem 0;
}
.video-grid h2 {
  color: rgb(var(--vnx-pink));
  font-family: var(--font-vnx);
  text-align: left;
  font-size: 3rem;
}
.channel-section {
  margin-bottom: 2.5rem;
  padding-bottom: 2rem;
}
.channel-header {
  margin-bottom: 0.2rem;
}
.channel-title {
  font-size: 1.35rem;
  color: rgb(var(--vnx-white));
  font-weight: bold;
  margin-bottom: 0.1rem;
  font-family: var(--font-vnx);
  text-transform: capital;
}
.channel-title a {
  background: -webkit-linear-gradient(
    0deg,
    rgb(var(--vnx-pink)) 50%,
    rgb(var(--vnx-fg)) 50%
  );
  background-position: 100% 100%;
  background-size: 200% 100%;
  padding: 0.1rem 1.7rem 0.1rem 0.3rem;
  border-left: 2px solid rgb(var(--vnx-pink));
  color: rgb(var(--vnx-white));
  text-decoration: none;
  text-align: left;
  transition: text-decoration 0.2s, background-position 0.2s;
}
.channel-title a:hover {
  text-decoration: underline;
  background-position: 0% 0%;
}
.channel-desc {
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
}
.side-scroll-grid:hover {
  scrollbar-color: #0000;
}
.side-scroll-grid::-webkit-scrollbar {
  height: 12px;
}
.side-scroll-grid::-webkit-scrollbar-thumb {
  background: #0000;
  border-radius: 0;
}
.side-scroll-grid:hover::-webkit-scrollbar-thumb {
  background: #0000;
}
.side-scroll-grid::-webkit-scrollbar-track {
  border-radius: 0;
}
.video-card {
  position: relative;
  margin-top: 2.8rem;
  border-radius: 0;
  overflow: hidden;
  width: 320px;
  max-width: 90vw;
  text-align: center;
  transition: transform 0.15s;
  flex: 0 0 320px;
  padding-top: 2.7rem;
  background: rgb(var(--vnx-fg));
  border-radius: 12px;
  border-top: 4px solid rgb(var(--vnx-pink));
  border-bottom: 4px solid rgb(var(--vnx-black));
  font-family: var(--font-vnx);
  transition: transform 0.4s ease;
}
.video-card:hover {
  transform: translateY(-10%);
}
.video-card img {
  width: 100%;
  display: block;
}
.video-embed {
  width: 100%;
  aspect-ratio: 16/9;
  margin-bottom: 0.5rem;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
  background: #000;
  display: block;
}
.video-iframe {
  width: 100%;
  height: 100%;
  display: block;
  aspect-ratio: 16/9;
  background: #000;
}

.video-title {
  display: block;
  margin-bottom: 1.0rem;
  padding: 0.5rem 0;
  color: rgb(var(--vnx-white));
  font-weight: bold;
  text-decoration: none;
  transition: color 0.2s;
  text-align: left;
  font-family: var(--font-mc);
  letter-spacing: 1px;
}
.video-title:hover {
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
}

.video-stats {
  color: rgba(var(--vnx-white), 0.8);
  font-weight: bold;
  display: grid;
  grid-template-columns: repeat(3, auto);
  padding: 0 0.6rem 0 0.6rem;
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  margin: 0 0 1rem 0;
}

</style> 
