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
      <span v-if="playing">⏹</span>
      <span v-else>▶</span>
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
  min-width: 100px;
  width: 100%;
  font-family: var(--font-vnx);
}
.audio-btn {
  font: var(--font-vnx);
  background: rgb(var(--vnx-mg));
  width: 44px;
  height: 44px;
  font-size: 1.5rem;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s, color 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 16px;
}
.audio-btn:hover {
  background: rgb(var(--vnx-pink));
}

.audio-progress {
  flex: 1 1 0%;
  min-width: 0;
  height: 12px;
  background: rgb(var(--vnx-mg));
  border-radius: 16px;
}
.audio-progress::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 16px;
  cursor: pointer;
  transition: background 0.2s;
}
.audio-progress:focus::-webkit-slider-thumb {
}
.audio-progress::-webkit-slider-runnable-track {
  height: 12px;
  border-radius: 16px;
}
.audio-progress::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 16px;
  cursor: pointer;
  transition: background 0.2s;
  background: rgb(var(--vnx-pink));
  border: none;
}
.audio-progress:focus::-moz-range-thumb {
  background: rgb(var(--vnx-highlight));
}
.audio-progress::-ms-thumb {
  background: rgb(var(--vnx-pink));
  width: 18px;
  height: 18px;
  cursor: pointer;
  transition: background 0.2s;
  border: none;
}
.audio-progress:focus::-ms-thumb {
  background: rgb(var(--vnx-highlight));
}
.audio-time {
  color: rgb(var(--vnx-pink));
  flex-shrink: 0;
  min-width: 60px;
  font-size: 0.8em;
  text-align: right;
  margin-left: 0.5rem;
  letter-spacing: 0.2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-family: var(--font-vnx);
}
.audio-time-sep {
  font-weight: bold;
  color: rgba(var(--vnx-pink), 0.7);
}
</style> 
