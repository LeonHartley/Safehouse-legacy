<template>
  <div class="contacts-bar">
    <add-contact></add-contact>
    <ul class="contacts">
      <li v-for="(contact, id, index) in contacts" 
        :key="index" 
        v-on:click="selectContact(id)"
        :class="{'active': activeContact != null && activeContact.id == id}">
            <img :src="contact.avatar" :class="{
              'contact-img': true,
              'status-online': contact.status == 'online',
              'status-offline': contact.status == 'offline'
            }"/>
      </li>
    </ul>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import AddContact from './AddContact.vue'

  export default {
    name: 'contacts-bar',

    methods: {
      selectContact (id) {
        this.$store.commit('setActiveContact', {
          activeContact: this.contacts[id]
        })

        this.$store.commit('startChat', id)
      }
    },

    computed: {
      ...mapState({
        chats: state => state.Chat.chats,
        contacts: state => state.User.contacts,
        activeContact: state => state.User.activeContact
      })
    },

    components: {
      AddContact
    }
}
</script>

<style>
  .contacts-bar {
    position: realtive;
  }
</style>