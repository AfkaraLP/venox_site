<script setup lang="ts">
import kateyByrd from '../../assets/images/team/katey-byrd.png'
import kateyByrdChar from '../../assets/images/team/katey-byrd-char.png'
import diaGrey from '../../assets/images/team/dia-grey.png'
import diaGreyChar from '../../assets/images/team/dia-grey-char.png'
import jeremiahVang from '../../assets/images/team/jeremiah-vang.png'
import jeremiahVangChar from '../../assets/images/team/jeremiah-vang-char.png'
import kyleLambky from '../../assets/images/team/kyle-lambky.png'
import kyleLambkyChar from '../../assets/images/team/kyle-lambky-char.png'
import harmonSounds from '../../assets/images/team/harmonsounds.png'

interface TeamMember {
  name: string
  role: string
  link: string
  image: string
  charImage?: string
}

const team: TeamMember[] = [
  { name: 'Katey Byrd', role: 'Venox', link: 'https://www.instagram.com/kateybyrd_va/', image: kateyByrd, charImage: kateyByrdChar },
  { name: 'Dia Grey', role: 'Tsumigi', link: 'https://diagrey.carrd.co/', image: diaGrey, charImage: diaGreyChar },
  { name: 'Jeremiah Vang', role: 'Yasuo', link: 'https://linktr.ee/jervoices', image: jeremiahVang, charImage: jeremiahVangChar },
  { name: 'Kyle Lambky', role: 'Yuji', link: 'https://www.kylelambky.com/', image: kyleLambky, charImage: kyleLambkyChar },
  { name: 'HarmonSounds', role: 'the Sound Designer', link: 'https://x.com/Harmon1697198', image: harmonSounds },
]
</script>

<template>
  <section class="team-section">
    <h2>The Team</h2>
    <div class="team-grid">
      <a
        v-for="member in team"
        :key="member.name"
        :href="member.link"
        target="_blank"
        rel="noopener noreferrer"
        class="team-card"
      >
        <div class="team-photo-wrap">
          <div class="team-photos" :class="{ 'single': !member.charImage }">
            <img :src="member.image" :alt="member.name" class="team-photo" loading="lazy" />
            <img v-if="member.charImage" :src="member.charImage" :alt="`${member.name} as ${member.role}`" class="team-photo" loading="lazy" />
          </div>
        </div>
        <div class="team-info">
          <span class="team-name">{{ member.name }}</span>
          <span class="team-role">as {{ member.role }}</span>
        </div>
      </a>
    </div>
  </section>
</template>

<style scoped>
.team-section {
  margin: 0 0 3rem 0;
}
.team-section h2 {
  margin-bottom: 1.5rem;
  text-align: left;
  color: rgb(var(--vnx-pink));
  font-family: var(--font-vnx);
  font-size: 2rem;
}
.team-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 1.6rem;
}
.team-card {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: rgb(var(--vnx-fg));
  border-radius: 16px;
  overflow: hidden;
  text-decoration: none;
  transition: transform 0.2s, box-shadow 0.2s;
}
.team-card:hover {
  transform: translateY(-4%);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.45);
}
.team-photo-wrap {
  border-top: 4px solid rgb(var(--vnx-pink));
  background: rgb(var(--vnx-mg));
  display: flex;
  align-items: stretch;
  justify-content: center;
}
.team-photos {
  display: grid;
  grid-template-columns: 1fr 1fr;
  width: 100%;
}
.team-photos.single {
  grid-template-columns: 1fr;
}
.team-photo {
  width: 100%;
  height: auto;
  display: block;
  aspect-ratio: 1 / 1;
  object-fit: cover;
}
.team-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 0.25rem;
  padding: 0.9rem 1.2rem 1.1rem 1.2rem;
  font-family: var(--font-vnx);
  border-bottom: 4px solid rgb(var(--vnx-black));
}
.team-name {
  color: rgb(var(--vnx-pink));
  font-weight: bold;
  font-size: 1.4rem;
}
.team-role {
  color: rgb(var(--vnx-white));
  font-size: 1.05rem;
  opacity: 0.85;
}
@media (max-width: 700px) {
  .team-grid {
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
  }
  .team-name {
    font-size: 1.15rem;
  }
  .team-role {
    font-size: 0.9rem;
  }
}
</style>
