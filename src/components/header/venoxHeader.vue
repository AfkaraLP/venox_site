<template>
  <header class="venox-header-mc">
    <div class="logo-container-mc">
      <a href="/">
        <span class="sr-only">Venox Home</span>
        <span class="site-title">LilVenox</span>
      </a>
    </div>
    <nav class="main-nav">
      <a href="#music" class="nav-link">Music</a>
      <a href="#videos" class="nav-link">Videos</a>
      <a href="#footer" class="nav-link">Contact</a>
    </nav>
  </header>
</template>


<style scoped>
a[href="/"] {
  text-decoration: none;
}

.venox-header-mc {
  border-bottom: 2px solid rgb(var(--vnx-pink));
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1.2rem 2.5rem 1.2rem 2.5rem;
  position: sticky;
  top: 0;
  z-index: 10;
  background: rgb(var(--vnx-black));
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
  background: -webkit-linear-gradient(
      315deg,
      rgb(var(--vnx-pink)) 40%,
      rgba(var(--vnx-white), 0.2) 40%,
      rgba(var(--vnx-white), 0.2) 60%,
      rgba(var(--vnx-white), 0.1) 60%,
      rgba(var(--vnx-white), 0.1) 80%
    );

  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-size: 200% 100%;
  background-position: 50% 1000%;  font-weight: bold;
  letter-spacing: 0.220rem;
  transition: background-position 0.8s ease;
}

.site-title:hover {
  background-position: 0% 0%;
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
