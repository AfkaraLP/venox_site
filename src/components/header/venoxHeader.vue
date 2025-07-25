<template>
  <header class="venox-header-mc">
    <div class="logo-container-mc">
      <a href="/">
        <span class="sr-only">Venox Home</span>
        <!-- <span class="logo-svg-mc" v-html="logoSvg"></span> -->
      </a>
      <span class="site-title-mc">LILVENOX</span>
    </div>
    <nav class="main-nav-mc">
      <a href="#music" class="nav-link-mc">Music</a>
      <a href="#videos" class="nav-link-mc">Videos</a>
      <!-- <a href="#art" class="nav-link-mc">Art</a> -->
      <a href="#footer" class="nav-link-mc">Contact</a>
    </nav>
  </header>
</template>

<script setup lang="ts">
// const logoSvg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 261.76 226.69" width="38" height="38"><path d="M161.096.001l-30.225 52.351L100.647.001H-.005l130.877 226.688L261.749.001z" fill="#41b883"/><path d="M161.096.001l-30.225 52.351L100.647.001H52.346l78.526 136.01L209.398.001z" fill="#34495e"/></svg>`
</script>

<style scoped>
.venox-header-mc {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.2rem 2.5rem 1.2rem 2.5rem;
  background: url('/src/assets/images/minecraft_grass.png') repeat, #6aac4b;
  border-bottom: 6px double #222;
  border-top: 6px double #222;
  box-shadow: 0 8px 0 #4e2e13;
  border-radius: 0;
  position: sticky;
  top: 0;
  z-index: 10;
  image-rendering: pixelated;
}
.logo-container-mc {
  display: flex;
  align-items: center;
  gap: 1.1rem;
}
.logo-svg-mc {
  display: inline-block;
  vertical-align: middle;
  width: 38px;
  height: 38px;
  image-rendering: pixelated;
}
.site-title-mc {
  font-family: var(--font-mc);
  font-size: 2.2rem;
  color: #222;
  font-weight: bold;
  letter-spacing: 2px;
  text-transform: uppercase;
  text-shadow: 2px 2px 0 #fff, 4px 4px 0 #4e2e13;
  outline: 2px solid #fff8;
  outline-offset: 2px;
  filter: drop-shadow(0 0 2px #fff8);
}
.main-nav-mc {
  display: flex;
  gap: 2.2rem;
}
.nav-link-mc {
  font-family: var(--font-mc);
  color: #fff;
  background: #4e2e13;
  font-size: 1.15rem;
  text-decoration: none;
  font-weight: bold;
  text-transform: uppercase;
  border: 4px double #222;
  border-radius: 0;
  padding: 0.3rem 1.1rem;
  box-shadow: 0 4px 0 #222;
  transition: background 0.2s, color 0.2s, transform 0.15s;
  letter-spacing: 1px;
  image-rendering: pixelated;
}
.nav-link-mc:hover {
  background: #6aac4b;
  color: #222;
  transform: scale(1.07) rotate(-2deg);
  box-shadow: 0 8px 0 #4e2e13;
}
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0,0,0,0);
  border: 0;
}
@media (max-width: 900px) {
  .venox-header-mc {
    flex-direction: column;
    align-items: flex-start;
    padding: 1rem 1.2rem;
  }
  .main-nav-mc {
    gap: 1.1rem;
    margin-top: 0.7rem;
  }
  .site-title-mc {
    font-size: 1.3rem;
  }
}
</style>
