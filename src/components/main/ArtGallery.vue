<template>
  <section class="art-gallery">
    <h2>Art Gallery</h2>
    <div class="gallery-grid">
      <div class="art-card" v-for="n in 3" :key="n">
        <img :src="`https://placehold.co/240x320?text=Art+${n}`" :alt="`Art ${n}`" />
        <div class="art-title">Genshin Fanart #{{ n }}</div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.art-gallery {
  margin: 2rem 0;
}
.art-gallery h2 {
  color: #5a4fcf;
  margin-bottom: 1rem;
  text-align: left;
}
.gallery-grid {
  display: flex;
  gap: 1.5rem;
  flex-wrap: wrap;
  justify-content: center;
}
.art-card {
  background: #fff;
  border-radius: 1rem;
  box-shadow: 0 2px 12px #b5aaff22;
  overflow: hidden;
  width: 240px;
  max-width: 90vw;
  text-align: center;
  transition: transform 0.15s;
}
.art-card:hover {
  transform: translateY(-6px) scale(1.03);
}
.art-card img {
  width: 100%;
  display: block;
}
.art-title {
  padding: 0.5rem 0;
  color: #7b6ee6;
  font-weight: bold;
}
</style> 