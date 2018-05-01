<template>
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
</template>

<script>
  import { mapState } from 'vuex'
  
  export default {
    name: 'contacts-bar',

    methods: {
      selectContact (id) {
        this.$store.commit('setActiveContact', {
          activeContact: this.contacts[id]
        })
      }
    },

    computed: {
      ...mapState({
        contacts: state => state.User.contacts,
        activeContact: state => state.User.activeContact
      })
    }
}
</script>