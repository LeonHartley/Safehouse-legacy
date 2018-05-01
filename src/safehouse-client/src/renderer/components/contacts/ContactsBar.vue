<template>
  <ul class="contacts">
    <li v-for="(contact, index) in contacts" 
      :key="index" 
      v-on:click="selectContact(contact.id)"
      :class="{'active': activeContact != null && activeContact.id == contact.id}">
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
          activeContact: this.contactsObj[id]
        })
      }
    },

    computed: {
      contacts () {
        var contacts = []
        let contactsObj = this.contactsObj

        Object.keys(contactsObj).forEach(function (id) {
          contacts.push(contactsObj[id])
        })

        return contacts
      },
      ...mapState({
        contactsObj: state => state.User.contacts,
        activeContact: state => state.User.activeContact
      })
    }
}
</script>