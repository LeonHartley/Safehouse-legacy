<template>
  <div class="main" v-if="activeContact != null">
    <div class="main-contact" >
      <img :src="activeContact.avatar" :class="{
        'contact-img': true,
        'contact-img-large': true,
        'status-online': activeContact.status == 'online',
        'status-offline': activeContact == null ||activeContact.status == 'offline'
      }" />
      
      <span class="contact-name">{{ activeContact.username }}</span>
    </div>
    <div class="main-chat">

    </div>
  </div>
  <div class="main" v-else>
    No chat selected :(
  </div>
</template>

<script>
  import { mapState } from 'vuex'

  export default {
    name: 'active-chat',
    methods: {
    },

    computed: {
      ...mapState({
        chat: state => state.Chat.activeChat,
        activeContact: state => state.User.activeContact
      })
    }
  }
</script>

<style>

</style>
