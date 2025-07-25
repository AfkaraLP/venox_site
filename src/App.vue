<script setup lang="ts">
import { ref, onMounted } from 'vue'
import VenoxHeader from "./components/header/venoxHeader.vue"
import HomeHero from "./components/main/HomeHero.vue"
import FeaturedVideo from "./components/main/FeaturedVideo.vue"
import VideoGrid from "./components/main/VideoGrid.vue"
import MusicSection from "./components/main/MusicSection.vue"
// import ArtGallery from "./components/main/ArtGallery.vue" // still hidden

const featuredVideo = ref<any>(null)
const featuredVideoError = ref<string | null>(null)
const videoGridLoading = ref(true)

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
function getThumbnail(entry: any) {
  return (
    entry.media_group?.media_thumbnail?.url ||
    entry.group?.thumbnail?.url ||
    ''
  )
}

onMounted(async () => {
  try {
    const res = await fetch('/api/youtube_videos')
    const feeds = await res.json()
    let allVideos = feeds.flatMap((feed: any) => Array.isArray(feed.entry) ? feed.entry : [])
    allVideos = allVideos.filter((v: any) => v && v.id && v.title && v.published)
    allVideos.sort((a: any, b: any) => {
      const dateA = new Date(a.published).getTime()
      const dateB = new Date(b.published).getTime()
      return dateB - dateA
    })
    const latest = allVideos[0]
    if (latest && latest.id && getThumbnail(latest)) {
      featuredVideo.value = {
        id: latest.id,
        title: latest.title,
        url: getYoutubeUrl(latest.id),
        embedUrl: getEmbedUrl(latest.id),
        thumbnail: getThumbnail(latest)
      }
      featuredVideoError.value = null
    } else {
      featuredVideo.value = null
      featuredVideoError.value = null
    }
  } catch (e) {
    featuredVideo.value = null
    featuredVideoError.value = 'No featured video available.'
  } finally {
    videoGridLoading.value = false
  }
})
</script>

<template>
  <VenoxHeader />
  <main class="main-content">
    <HomeHero />
    <FeaturedVideo
      :video="featuredVideo"
      :loading="videoGridLoading"
      :key="featuredVideo?.id || 'skeleton'"
    />
    <div v-if="featuredVideoError" class="error" style="text-align:center; color:#b00; margin:2rem 0;">
      {{ featuredVideoError }}
    </div>
    <div id="videos">
      <VideoGrid />
    </div>
    <!-- <ArtGallery /> -->
    <div id="music">
      <MusicSection />
    </div>
  </main>

  <footer id="footer">
    <div class="footer-content">
      <div class="socials">
        <!-- <a href="https://www.twitch.tv/lilvenoxofficial" class="social-link" target="_blank" rel="noopener" title="Twitch"> -->
          <!-- <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/twitch.svg" alt="Twitch" width="28" height="28" /> -->
        <!-- </a> -->
        <a href="https://www.instagram.com/lilvenoxmc/" class="social-link" target="_blank" rel="noopener" title="Instagram">
          <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/instagram.svg" alt="Instagram" width="28" height="28" />
        </a>
        <a href="https://discord.gg/hew9GxbbQU" class="social-link" target="_blank" rel="noopener" title="Discord">
          <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/discord.svg" alt="Discord" width="28" height="28" />
        </a>
        <a href="https://www.youtube.com/channel/UCs9v8InFppRaPtTJjOpxWWg/membership" class="social-link" target="_blank" rel="noopener" title="YouTube">
          <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/youtube.svg" alt="YouTube" width="28" height="28" />
        </a>
        <!-- <a href="https://www.tiktok.com/@lilvenox" class="social-link" target="_blank" rel="noopener" title="TikTok"> -->
          <!-- <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/tiktok.svg" alt="TikTok" width="28" height="28" /> -->
        <!-- </a> -->
        <!-- <a href="https://x.com/lil_venox" class="social-link" target="_blank" rel="noopener" title="X (Twitter)"> -->
          <!-- <img src="https://cdn.jsdelivr.net/npm/simple-icons@v11/icons/x.svg" alt="X (Twitter)" width="28" height="28" /> -->
        <!-- </a> -->
      </div>
      <span>© 2025 LilVenox</span>
    </div>
  </footer>
</template>
