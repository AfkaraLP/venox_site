<template>
  <button
    class="mc-play-btn"
    :style="{
      width: size + 'px',
      height: size + 'px',
      fontSize: (size * 0.6) + 'px',
      background: bgColor,
      color: color
    }"
    @click="$emit('click')"
    :aria-label="ariaLabel"
  >
    ▶
  </button>
</template>

<script setup lang="ts">
defineProps({
  size: { type: Number, default: 44 },
  color: { type: String, default: '#fff' },
  bgColor: { type: String, default: '#7e5c3e' }, // Minecraft brown
  ariaLabel: { type: String, default: 'Play' }
})
</script>

<style scoped>
.mc-play-btn {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  border: 4px double #222;
  border-radius: 0;
  cursor: pointer;
  box-shadow: 0 4px 0 #4e2e13;
  transition: background 0.2s, color 0.2s;
  z-index: 2;
  image-rendering: pixelated;
  font-family: var(--font-mc);
  font-weight: bold;
  user-select: none;
  display: flex;
  align-items: center;
  justify-content: center;
}
.mc-play-btn:hover {
  background: #4e2e13 !important;
  color: #6aac4b !important;
}
</style> 