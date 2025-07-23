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
  background: linear-gradient(90deg, #b5aaff 0%, #8ec5fc 100%);
  border-radius: 1rem;
  padding: 0.7rem 1.1rem;
  min-width: 0;
  width: 100%;
  box-sizing: border-box;
}
.audio-btn {
  flex-shrink: 0;
  background: #fff;
  border: none;
  border-radius: 50%;
  width: 44px;
  height: 44px;
  font-size: 1.5rem;
  color: #7b6ee6;
  box-shadow: 0 1px 4px #b5aaff22;
  cursor: pointer;
  transition: background 0.2s, color 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}
.audio-btn:hover {
  background: #e0c3fc;
  color: #5a4fcf;
}
.audio-progress {
  flex: 1 1 0%;
  min-width: 0;
  accent-color: #7b6ee6;
  height: 8px;
  border-radius: 1rem;
  background: #e0c3fc;
  margin: 0 0.7rem;
  outline: none;
}
.audio-progress::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #7b6ee6;
  border: 2.5px solid #fff;
  box-shadow: 0 1px 4px #b5aaff44;
  cursor: pointer;
  transition: background 0.2s;
}
.audio-progress:focus::-webkit-slider-thumb {
  background: #3a2e8c;
}
.audio-progress::-webkit-slider-runnable-track {
  height: 8px;
  border-radius: 1rem;
  background: #e0c3fc;
}
.audio-progress::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #7b6ee6;
  border: 2.5px solid #fff;
  box-shadow: 0 1px 4px #b5aaff44;
  cursor: pointer;
  transition: background 0.2s;
}
.audio-progress:focus::-moz-range-thumb {
  background: #3a2e8c;
}
.audio-progress::-ms-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #7b6ee6;
  border: 2.5px solid #fff;
  box-shadow: 0 1px 4px #b5aaff44;
  cursor: pointer;
  transition: background 0.2s;
}
.audio-progress:focus::-ms-thumb {
  background: #3a2e8c;
}
.audio-time {
  flex-shrink: 0;
  min-width: 60px;
  color: #7b6ee6;
  font-size: 1em;
  text-align: right;
  margin-left: 0.5rem;
  letter-spacing: 0.2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.audio-time-sep {
  margin: 0 0.2em;
  color: #b5aaff;
  font-weight: bold;
}
.track-title {
  color: #7b6ee6;
  font-weight: bold;
}
.track-cover {
  width: 48px;
  height: 48px;
  border-radius: 0.5rem;
  object-fit: cover;
  box-shadow: 0 1px 6px #b5aaff22;
}
</style> 