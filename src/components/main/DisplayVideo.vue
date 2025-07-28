<template>
  <div class="aspect-video bg-pink-prim w-2xl rounded-3xl">
    <p>youtube video</p>
  </div>
</template>

<script setup lang="ts">
// const google_api_key = import.meta.env.VITE_GOOGLE_API_KEY;
// import google from '@googleapis/youtube';
// const youtube = google.youtube('v3');
import { youtube } from 'scrape-youtube';

const options = {
  type: 'video'
};

const videos = await youtube.search("PiccoloVenox", options);

</script>

<style scoped></style>
