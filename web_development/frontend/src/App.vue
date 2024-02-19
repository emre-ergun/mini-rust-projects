<script lang="ts">
import { ref } from 'vue';
const message = ref(null);

async function getMessage() {
  const response = await fetch('http://localhost:3030');
  const json = await response.json();
  message.value = json.message;
}
</script>

<template>
  <h1>Rust + Vue</h1>
  <div v-if="message">
    Received: {{ message }}
  </div>
  <div v-else>
    No Message
  </div>
  <button @click="getMessage()">Get Message</button>
</template>