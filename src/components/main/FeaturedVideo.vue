<script setup lang="ts">
import { ref, computed } from 'vue'
import PlayButton from './PlayButton.vue'
const props = defineProps<{
  video?: {
    id: string,
    title: string,
    url: string,
    embedUrl: string,
    thumbnail: string,
  }
  loading?: boolean
}>()

const playing = ref(false)

function playVideo() {
  playing.value = true
}

const showSkeleton = computed(() => props.loading || !props.video || !props.video.title || !props.video.thumbnail)
</script>

<template>
  <section class="featured-video">
    <template v-if="!showSkeleton">
      <div class="video-thumb">
        <template v-if="!playing">
          <img :src="props.video.thumbnail" :alt="props.video.title" />
          <PlayButton :size="80" @click="playVideo" />
        </template>
        <template v-else>
          <iframe
            :src="props.video.embedUrl + '?autoplay=1'"
            frameborder="0"
            allowfullscreen
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
            title="YouTube video player"
            class="video-iframe"
          ></iframe>
        </template>
      </div>
      <div class="video-info">
        <div class="video-title-overlay" v-if="!playing">
          {{ props.video.title }}
        </div>
        <a :href="props.video.url" target="_blank" rel="noopener noreferrer" class="youtube-link">
          Watch on YouTube
        </a>
      </div>
    </template>
    <template v-else>
      <div class="featured-skeleton">
        <div class="skeleton-thumb"></div>
        <div class="skeleton-title"></div>
        <div class="skeleton-btn"></div>
      </div>
    </template>
  </section>
</template>

<style scoped>
.featured-video {
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
}
.video-title-overlay {
  color: #fff;
  font-size: 1.35rem;
  font-weight: 600;
  padding: 0.5rem 1.2rem;
  z-index: 2;
  max-width: 90vw;
  text-align: left;
  margin-bottom: 2rem;
}
.video-thumb {
  position: relative;
  width: 900px;
  max-width: 98vw;
  aspect-ratio: 16/9;
  border-radius: 2rem;
  overflow: hidden;
  background: #222; /* fallback */
  display: block;
}
.video-thumb img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}
.video-iframe {
  width: 100%;
  height: 100%;
  border: none;
  display: block;
  aspect-ratio: 16/9;
  background: #000;
}
.video-info {
  margin-top: 0.7rem;
  text-align: center;
}
.youtube-link {
  color: rgb(var(--vnx-pink));
  font-size: 1.25rem;
  font-weight: bold;
  text-decoration: none;
  background: rgb(var(--vnx-fg));
  padding: 0.7rem 2.2rem;
  border-radius: 8px;
  border-top: solid 2px rgb(var(--vnx-pink));
  border-bottom: solid 2px rgb(var(--vnx-black));
  transition: background 0.2s, color 0.2s, border-top 0.2s;
}
.youtube-link:hover {
  background: rgb(var(--vnx-fg));
  color: rgb(var(--vnx-highlight));
  border-top: solid 2px rgb(var(--vnx-highlight));
  text-decoration: underline;
}
/* Skeleton Loader Styles */
.featured-skeleton {
  width: 900px;
  max-width: 98vw;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.2rem;
  margin-bottom: 1.2rem;
}
.skeleton-thumb {
  width: 100%;
  aspect-ratio: 16/9;
  border-radius: 2rem;
  background: linear-gradient(90deg, #e0c3fc22 25%, #b5aaff33 50%, #e0c3fc22 75%);
  background-size: 200% 100%;
  animation: skeleton-loading 1.2s infinite linear;
}
.skeleton-title {
  width: 60%;
  height: 2.2rem;
  border-radius: 1.2rem;
  background: linear-gradient(90deg, #e0c3fc33 25%, #b5aaff44 50%, #e0c3fc33 75%);
  background-size: 200% 100%;
  animation: skeleton-loading 1.2s infinite linear;
}
.skeleton-btn {
  width: 120px;
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
@media (max-width: 1000px) {
  .video-thumb, .featured-skeleton {
    width: 98vw;
    min-width: 0;
    border-radius: 1.2rem;
  }
  .video-title-overlay {
    font-size: 1.1rem;
    padding: 0.5rem 1.2rem;
  }
}
</style> 
