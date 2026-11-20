<template>
  <div class="side-dock" :class="{ active }" aria-hidden="false">
    <a
      class="dock-item dock-plush"
      href="https://www.plushshop.com/products/venoxmeow"
      target="_blank"
      rel="noopener noreferrer"
      title="Venoxmeow plush — Plushshop"
      aria-label="Venoxmeow plush on Plushshop (opens in a new tab)"
    >
      <img
        src="https://www.plushshop.com/cdn/shop/files/venoxmeow-pre-order-ship-by-nov-15-dec-wan-ou-375.webp?v=1791303447&width=400"
        alt="Venoxmeow plush"
        loading="lazy"
      />
    </a>
    <a
      class="dock-item dock-merch"
      href="https://lilvenox.dashery.com/"
      target="_blank"
      rel="noopener noreferrer"
      title="Venox merch store — Dashery"
      aria-label="Venox merch store on Dashery (opens in a new tab)"
    >
      <img
        :src="shirtBack"
        alt="Venox T-Shirt, back view"
        loading="lazy"
      />
    </a>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue"
import shirtBack from "../../assets/images/venox-shirt-back.png"

const active = ref(false)
let observer: IntersectionObserver | null = null

onMounted(() => {
  const row = document.querySelector(".shop-row")
  if (!row || typeof IntersectionObserver === "undefined") return
  observer = new IntersectionObserver(
    (entries) => {
      const e = entries[0]
      // dock once the shop row has scrolled ABOVE the viewport
      active.value = !e.isIntersecting && e.boundingClientRect.top < 0
    },
    { threshold: 0 }
  )
  observer.observe(row)
})

onBeforeUnmount(() => observer?.disconnect())
</script>

<style scoped>
.side-dock {
  position: fixed;
  z-index: 60;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  flex-direction: column;
  gap: 16px;
  pointer-events: none;
}
.dock-item {
  pointer-events: auto;
  display: block;
  width: 150px;
  margin-right: -84px; /* clipped by the screen edge */
  padding: 10px 12px;
  border: 1px solid rgba(255, 255, 255, .28);
  border-right: none;
  border-radius: 16px 0 0 16px;
  background: linear-gradient(305deg, rgba(23, 19, 28, .96), rgba(58, 17, 48, .96));
  box-shadow: 0 10px 30px rgba(53, 10, 34, .38);
  transform: translateX(0);
  transition: transform .25s ease, box-shadow .25s ease;
}
.dock-item:hover,
.dock-item:focus-visible {
  transform: translateX(-96px);
  box-shadow: 0 16px 40px rgba(53, 10, 34, .5);
}
.dock-item img {
  display: block;
  width: 100%;
  max-height: 150px;
  object-fit: contain;
  filter: drop-shadow(0 6px 10px rgba(0, 0, 0, .4));
}
/* fade/slide in only after the shop row scrolled out of view */
.side-dock .dock-item {
  opacity: 0;
  translate: 40px 0;
  transition: transform .25s ease, box-shadow .25s ease, opacity .3s ease, translate .3s ease;
}
.side-dock.active .dock-item {
  opacity: 1;
  translate: 0 0;
}
@media (max-width: 1080px) {
  .side-dock { display: none; }
}
@media (prefers-reduced-motion: reduce) {
  .dock-item, .side-dock .dock-item { transition: none; }
}
</style>
