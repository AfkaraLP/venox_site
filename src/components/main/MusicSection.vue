<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'
import CustomAudioPlayer from './CustomAudioPlayer.vue'

interface Song {
  title: string
  pub_date: string
  link: string
  duration: string
  description: string
  enclosure?: {
    url: string
    data_enclosure_type?: string
  }
  data_enclosure?: {
    url: string
    data_enclosure_type?: string
  }
  image?: {
    url?: string
  }
}

const tracks = ref<Song[]>([])
const loading = ref(true)
const error = ref<string | null>(null)
const musicListRef = ref<HTMLElement | null>(null)
const sectionRef = ref<HTMLElement | null>(null)

// Track if mouse is over the very top section
let mouseOverTopSection = false

function getAudioUrl(track: Song): string | undefined {
  return track.enclosure?.url || track.data_enclosure?.url
}

function handleImageError(track: any) {
  track._imageError = true;
  console.error('Failed to load cover image for track:', track.title, track.image?.url);
}

function handleScrollHijack(e: WheelEvent) {
  const el = musicListRef.value
  if (!el) return
  // Always scroll horizontally when wheel event occurs on this section
  el.scrollLeft += e.deltaY
  e.preventDefault()
  // Optionally, you can log for debugging
  // console.log('[MUSIC SCROLL] Forced horizontal scroll', el.scrollLeft)
}

function isMusicSectionInView() {
  if (!sectionRef.value) return false
  const rect = sectionRef.value.getBoundingClientRect()
  // Consider in view if at least 60px of the section is visible
  return rect.bottom > 60 && rect.top < window.innerHeight - 60
}

function isFooterMostlyInView() {
  const footer = document.querySelector('footer')
  if (!footer) return false
  const rect = footer.getBoundingClientRect()
  const footerHeight = rect.height
  const visibleTop = Math.max(rect.top, 0)
  const visibleBottom = Math.min(rect.bottom, window.innerHeight)
  const visibleHeight = Math.max(0, visibleBottom - visibleTop)
  return visibleHeight / footerHeight >= 0.6
}

function handleGlobalWheel(e: WheelEvent) {
  if (mouseOverTopSection) return
  if (!isMusicSectionInView()) return
  if (!isFooterMostlyInView()) return
  const el = musicListRef.value
  if (!el) return
  // Only hijack if not at the very left or right
  const atStart = el.scrollLeft === 0 && e.deltaY < 0
  const atEnd = Math.abs(el.scrollLeft + el.clientWidth - el.scrollWidth) < 2 && e.deltaY > 0
  if (atStart || atEnd) return // allow normal vertical scroll at edges
  el.scrollLeft += e.deltaY * 2 // make it faster
  e.preventDefault()
}

onMounted(async () => {
  try {
    const res = await fetch('/api/soundcloud_data')
    if (!res.ok) throw new Error('Failed to fetch music')
    const feed = await res.json()
    tracks.value = Array.isArray(feed.channel?.item) ? feed.channel.item : []
    tracks.value.forEach(track => {
      console.log('Track image URL:', track.title, track.image)
    })
  } catch (e: any) {
    error.value = e.message || 'Unknown error'
    console.error('MusicSection fetch error:', e)
  } finally {
    loading.value = false
  }
  await nextTick()
  // Add global wheel event listener
  window.addEventListener('wheel', handleGlobalWheel, { passive: false })
  // Track mouse over top section
  const topSection = document.querySelector('.home-hero')
  if (topSection) {
    topSection.addEventListener('mouseenter', () => { mouseOverTopSection = true })
    topSection.addEventListener('mouseleave', () => { mouseOverTopSection = false })
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('wheel', handleGlobalWheel)
  const topSection = document.querySelector('.home-hero')
  if (topSection) {
    topSection.removeEventListener('mouseenter', () => { mouseOverTopSection = true })
    topSection.removeEventListener('mouseleave', () => { mouseOverTopSection = false })
  }
})
</script>

<template>
  <section class="music-section" ref="sectionRef">
    <h2>Music</h2>
    <div v-if="loading">
      <div class="music-list">
        <div class="music-track" v-for="n in 3" :key="'skeleton-' + n">
          <div class="track-header">
            <div class="skeleton-title"></div>
            <div class="skeleton-link"></div>
          </div>
          <div class="track-player">
            <div class="skeleton-audio"></div>
          </div>
        </div>
      </div>
    </div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else ref="musicListRef" class="music-list">
      <div class="music-track" v-for="track in tracks" :key="track.link"
        :style="track.image?.href && !track._imageError ? `background: linear-gradient(120deg, #e0c3fcbb 0%, #8ec5fcbb 100%), url('${track.image.href}') center/cover no-repeat;` : ''"
      >
        <div class="track-header">
          <span class="track-title">{{ track.title }}</span>
          <a :href="track.link" target="_blank" rel="noopener noreferrer" class="track-link">Open</a>
        </div>
        <div class="track-player">
          <CustomAudioPlayer
            v-if="getAudioUrl(track)"
            :src="getAudioUrl(track)"
            :title="undefined"
            :trackKey="track.link"
          />
          <span v-else class="audio-unavailable">Audio unavailable</span>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.music-section {
  margin: 2rem 0 3rem 0;
}
.music-section h2 {
  color: #5a4fcf;
  margin-bottom: 1.5rem;
  text-align: left;
}
.music-list {
  display: flex;
  flex-direction: row;
  gap: 2.2rem;
  align-items: stretch;
  overflow-x: auto;
  padding-bottom: 1rem;
  width: 100vw;
  min-width: 100vw;
  border: 2px dashed #b5aaff44;
  scroll-behavior: smooth;
}
.music-track {
  background: #fff;
  border-radius: 1.5rem;
  box-shadow: 0 4px 24px #b5aaff22;
  padding: 3rem 3.5rem 3.5rem 3.5rem;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: stretch;
  gap: 0;
  font-size: 1.22rem;
  min-width: 420px;
  max-width: 520px;
  min-height: 340px;
  transition: box-shadow 0.2s, transform 0.2s;
  position: relative;
  overflow: hidden;
  flex: 0 0 420px;
}
.music-track:hover {
  box-shadow: 0 8px 32px #b5aaff33;
  transform: translateY(-2px) scale(1.01);
}
.audio-unavailable {
  color: #aaa;
  font-size: 0.95em;
  margin-right: 1rem;
}
.track-link {
  color: #5a4fcf;
  text-decoration: none;
  font-size: 1.18rem;
  margin-left: auto;
  transition: color 0.2s;
  padding: 0.7rem 1.5rem;
  border-radius: 1rem;
  background: #f3eaff;
  box-shadow: 0 1px 4px #b5aaff11;
}
.track-link:hover {
  color: #7b6ee6;
  background: #e0c3fc;
  text-decoration: underline;
}
.track-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 2.2rem;
  background: rgba(255,255,255,0.82);
  border-radius: 1.2rem;
  padding: 0.7rem 1.2rem 0.7rem 1.2rem;
  box-shadow: 0 2px 8px #b5aaff11;
}
.track-title {
  color: #3a2e8c;
  font-weight: bold;
  font-size: 1.35em;
  margin-right: 0.5rem;
  letter-spacing: 0.2px;
}
.track-player {
  width: 100%;
  display: flex;
  align-items: flex-end;
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  padding: 0 2.5rem 1.2rem 2.5rem;
  background: linear-gradient(0deg, #e0c3fcbb 60%, transparent 100%);
  border-radius: 0 0 1.5rem 1.5rem;
  box-sizing: border-box;
}
.skeleton-title {
  width: 60%;
  height: 1.5rem;
  border-radius: 0.7rem;
  background: linear-gradient(90deg, #e0c3fc33 25%, #b5aaff44 50%, #e0c3fc33 75%);
  background-size: 200% 100%;
  animation: skeleton-loading 1.2s infinite linear;
  margin-bottom: 0.5rem;
}
.skeleton-link {
  width: 80px;
  height: 1.1rem;
  border-radius: 0.7rem;
  background: linear-gradient(90deg, #e0c3fc22 25%, #b5aaff33 50%, #e0c3fc22 75%);
  background-size: 200% 100%;
  animation: skeleton-loading 1.2s infinite linear;
}
.skeleton-audio {
  width: 100%;
  height: 2.2rem;
  border-radius: 1.2rem;
  background: linear-gradient(90deg, #e0c3fc33 25%, #b5aaff44 50%, #e0c3fc33 75%);
  background-size: 200% 100%;
  animation: skeleton-loading 1.2s infinite linear;
}
@keyframes skeleton-loading {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}
@media (max-width: 700px) {
  .music-list {
    gap: 1.2rem;
  }
  .music-track {
    flex-direction: column;
    align-items: stretch;
    padding: 1.5rem 0.7rem 2.5rem 0.7rem;
    gap: 1.2rem;
    min-width: 90vw;
    max-width: 98vw;
    min-height: 220px;
  }
  .track-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }
  .track-link {
    margin-left: 0;
    margin-top: 0.7rem;
    width: 100%;
    text-align: center;
  }
  .track-player {
    padding: 0 1rem 1.2rem 1rem;
  }
}
</style> 