<template>
  <header class="venox-header">
    <div class="logo-container">
      <a href="/">
        <span class="sr-only">Venox Home</span>
        <span class="logo-svg" v-html="logoSvg"></span>
      </a>
      <span class="site-title">LilVenox</span>
    </div>
    <nav class="main-nav">
      <a href="#music" class="nav-link">Music</a>
      <a href="#videos" class="nav-link">Videos</a>
      <a href="#art" class="nav-link">Art</a>
      <a href="#footer" class="nav-link">Contact</a>
    </nav>
  </header>
</template>

<script setup lang="ts">
const logoSvg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 261.76 226.69" width="38" height="38"><path d="M161.096.001l-30.225 52.351L100.647.001H-.005l130.877 226.688L261.749.001z" fill="#41b883"/><path d="M161.096.001l-30.225 52.351L100.647.001H52.346l78.526 136.01L209.398.001z" fill="#34495e"/></svg>`
</script>

<style scoped>
.venox-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.2rem 2.5rem 1.2rem 2.5rem;
  background: #fff;
  box-shadow: 0 2px 16px #b5aaff11;
  border-radius: 0 0 2rem 2rem;
  position: sticky;
  top: 0;
  z-index: 10;
}
.logo-container {
  display: flex;
  align-items: center;
  gap: 1.1rem;
}
.logo-svg {
  display: inline-block;
  vertical-align: middle;
  width: 38px;
  height: 38px;
}
.site-title {
  font-family: 'genshin', 'Segoe UI', Arial, sans-serif;
  font-size: 2rem;
  color: #5a4fcf;
  font-weight: bold;
  letter-spacing: 1px;
}
.main-nav {
  display: flex;
  gap: 2.2rem;
}
.nav-link {
  color: #7b6ee6;
  font-size: 1.15rem;
  text-decoration: none;
  font-weight: 500;
  transition: color 0.2s;
  padding: 0.3rem 0.7rem;
  border-radius: 1rem;
}
.nav-link:hover {
  color: #5a4fcf;
  background: #e0c3fc44;
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
  .venox-header {
    flex-direction: column;
    align-items: flex-start;
    padding: 1rem 1.2rem;
    border-radius: 0 0 1.2rem 1.2rem;
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
