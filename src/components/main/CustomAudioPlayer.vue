<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue'

const props = defineProps<{
  src: string
  cover?: string
  title?: string
  trackKey: string
}>()

const playing = ref(false)
const currentTime = ref(0)
const duration = ref(0)
const audioRef = ref<HTMLAudioElement | null>(null)

function formatTime(sec: number) {
  if (!isFinite(sec) || isNaN(sec) || sec < 0) return '0:00';
  const m = Math.floor(sec / 60)
  const s = Math.floor(sec % 60)
  return `${m}:${s.toString().padStart(2, '0')}`
}

function play() {
  // Pause all other audio elements on the page
  document.querySelectorAll('audio').forEach((el) => {
    if (el !== audioRef.value) el.pause()
  })
  audioRef.value?.play()
  playing.value = true
}

function pause() {
  audioRef.value?.pause()
  playing.value = false
}

function onTimeUpdate() {
  if (audioRef.value) {
    currentTime.value = audioRef.value.currentTime
    duration.value = audioRef.value.duration
  }
}

function onSeek(event: Event) {
  const input = event.target as HTMLInputElement
  if (audioRef.value && input) {
    audioRef.value.currentTime = Number(input.value)
    currentTime.value = audioRef.value.currentTime
  }
}

function onEnded() {
  playing.value = false
  currentTime.value = 0
}

watch(() => props.src, () => {
  // Reset state if src changes
  playing.value = false
  currentTime.value = 0
  duration.value = 0
  nextTick(() => {
    if (audioRef.value) audioRef.value.load()
  })
})
</script>

<template>
  <div class="custom-audio-player">
    <img v-if="props.cover" :src="props.cover" alt="cover" class="track-cover" />
    <span v-if="props.title" class="track-title">{{ props.title }}</span>
    <audio
      ref="audioRef"
      :src="props.src"
      @timeupdate="onTimeUpdate"
      @ended="onEnded"
      @pause="playing.value = false"
      @play="playing.value = true"
      preload="none"
      style="display:none"
    ></audio>
    <button
      class="audio-btn"
      @click="playing ? pause() : play()"
      :aria-label="playing ? 'Pause' : 'Play'"
    >
      <span v-if="playing">⏸️</span>
      <span v-else>▶️</span>
    </button>
    <input
      class="audio-progress"
      type="range"
      min="0"
      :max="duration || 0"
      step="0.1"
      :value="currentTime"
      @input="onSeek"
    />
    <span class="audio-time">
      {{ formatTime(currentTime) }}
      <span class="audio-time-sep">/</span>
      {{ formatTime(duration) }}
    </span>
  </div>
</template>

<style scoped>
.custom-audio-player {
  display: flex;
  align-items: center;
  gap: 0.7rem;
  background: #e3e3e3cc;
  border: 6px double #222;
  border-radius: 0;
  padding: 0.7rem 1.1rem;
  min-width: 0;
  width: 100%;
  box-sizing: border-box;
  font-family: var(--font-mc);
  image-rendering: pixelated;
  box-shadow: 0 8px 0 #4e2e13, 0 12px 0 #bcbcbc;
}
.audio-btn {
  flex-shrink: 0;
  background: #6aac4b;
  border: 4px double #222;
  border-radius: 0;
  width: 44px;
  height: 44px;
  font-size: 1.5rem;
  color: #fff;
  box-shadow: 0 4px 0 #4e2e13;
  cursor: pointer;
  transition: background 0.2s, color 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  image-rendering: pixelated;
}
.audio-btn:hover {
  background: #4e2e13;
  color: #6aac4b;
}
.audio-progress {
  flex: 1 1 0%;
  min-width: 0;
  accent-color: #6aac4b;
  height: 12px;
  border-radius: 0;
  background: #bcbcbc;
  margin: 0 0.7rem;
  outline: none;
  border: 4px dashed #222;
  box-shadow: 0 2px 0 #4e2e13;
  image-rendering: pixelated;
}
.audio-progress::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 0;
  background: #6aac4b;
  border: 4px double #222;
  box-shadow: 0 2px 0 #4e2e13;
  cursor: pointer;
  transition: background 0.2s;
  image-rendering: pixelated;
}
.audio-progress:focus::-webkit-slider-thumb {
  background: #4e2e13;
}
.audio-progress::-webkit-slider-runnable-track {
  height: 12px;
  border-radius: 0;
  background: #bcbcbc;
}
.audio-progress::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 0;
  background: #6aac4b;
  border: 4px double #222;
  box-shadow: 0 2px 0 #4e2e13;
  cursor: pointer;
  transition: background 0.2s;
  image-rendering: pixelated;
}
.audio-progress:focus::-moz-range-thumb {
  background: #4e2e13;
}
.audio-progress::-ms-thumb {
  width: 18px;
  height: 18px;
  border-radius: 0;
  background: #6aac4b;
  border: 4px double #222;
  box-shadow: 0 2px 0 #4e2e13;
  cursor: pointer;
  transition: background 0.2s;
  image-rendering: pixelated;
}
.audio-progress:focus::-ms-thumb {
  background: #4e2e13;
}
.audio-time {
  flex-shrink: 0;
  min-width: 60px;
  color: #4e2e13;
  font-size: 0.8em;
  text-align: right;
  margin-left: 0.5rem;
  letter-spacing: 0.2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: var(--font-mc);
  text-shadow: 2px 2px 0 #fff, 0 0 4px #bcbcbc;
}
.audio-time-sep {
  margin: 0 0.2em;
  color: #4e2e13;
  font-weight: bold;
}
.track-title {
  color: #4e2e13;
  font-weight: bold;
  font-family: var(--font-mc);
  text-shadow: 2px 2px 0 #fff, 0 0 4px #bcbcbc;
  text-transform: uppercase;
  letter-spacing: 1px;
}
.track-cover {
  width: 48px;
  height: 48px;
  border-radius: 0;
  object-fit: cover;
  box-shadow: 0 4px 0 #4e2e13;
  border: 4px double #222;
  image-rendering: pixelated;
}
</style> 