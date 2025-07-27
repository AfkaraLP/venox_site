<template>
  <header class="venox-header-mc">
    <div class="logo-container-mc">
      <a href="/">
        <span class="sr-only">Venox Home</span>
        <span class="site-title"><span class="site-title-pink">Lil</span>Venox</span>
      </a>
    </div>
    <nav class="main-nav">
      <a href="#music" class="nav-link">Music</a>
      <a href="#videos" class="nav-link">Videos</a>
      <a href="#footer" class="nav-link">Contact</a>
    </nav>
  </header>
</template>

<script setup lang="ts">
// const logoSvg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 261.76 226.69" width="38" height="38"><path d="M161.096.001l-30.225 52.351L100.647.001H-.005l130.877 226.688L261.749.001z" fill="#41b883"/><path d="M161.096.001l-30.225 52.351L100.647.001H52.346l78.526 136.01L209.398.001z" fill="#34495e"/></svg>`
</script>

<style scoped>
a[href="/"] {
  text-decoration: none;
}

.venox-header-mc {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.2rem 2.5rem 1.2rem 2.5rem;
  background: rgba(var(--vnx-black), 0.4);
  position: sticky;
  top: 0;
  z-index: 10;
  backdrop-filter: blur(8px);
}
.logo-container-mc {
  display: flex;
  align-items: center;
  gap: 1.1rem;
}
.site-title {
  position: relative;
  font-family: var(--font-vnx);
  font-size: 2.7rem;
  color: rgb(var(--vnx-black));
  font-weight: bold;
  -webkit-text-stroke: 4px rgba(var(--vnx-fg), 1);
  letter-spacing: 0.220rem;
}

.site-title-pink {
  color: rgb(var(--vnx-pink));
}

.main-nav {
  display: flex;
  gap: 2.2rem;
}
.nav-link {
  font-family: var(--font-vnx);
  color: #fff;
  font-size: 1.15rem;
  text-decoration: none;
  font-weight: bold;
  text-transform: capital;
  transition: background 0.2s, color 0.2s, transform 0.15s;
  letter-spacing: 1px;
  image-rendering: pixelated;
}
.nav-link:hover {
  transform: scale(1.07) rotate(-2deg);
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
  .main-nav {
    gap: 1.1rem;
    margin-top: 0.7rem;
  }
  .site-title {
    font-size: 1.3rem;
  }
}
</style>
