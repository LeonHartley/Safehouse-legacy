<template>
    <div class="main-container">
        <div class="side-bar">
            <ul class="contacts">
                <li v-for="(contact, index) in contacts" 
                  :key="index" 
                  v-on:click="selectContact(index)"
                  :class="{'active': activeContact != null && activeContact.id == contact.id}">
                     <img :src="contact.avatar" :class="{
                        'contact-img': true,
                        'status-online': contact.status == 'online',
                        'status-offline': contact.status == 'offline'
                      }"/>
                </li>
                
            </ul>
        </div>

        <div class="main">
            <div class="main-contact">
                <img :src="activeContact.avatar" :class="{
                  'contact-img': true,
                  'contact-img-large': true,
                  'status-online': activeContact.status == 'online',
                  'status-offline': activeContact.status == 'offline'
                }" />
                <span class="contact-name">{{ activeContact.username }}</span>
            </div>
            <div class="main-chat">
                 
            </div>
        </div>
    </div>
</template>

<script>
  import { mapState } from 'vuex'

  export default {
    name: 'chat',

    methods: {
      selectContact (index) {
        this.$store.commit('setActiveContact', {
          activeContact: this.contacts[index]
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

<style>

    .main-container {
        width: 100%;
        height: 100vh;
    }
    
    .side-bar {
        width: 70px;
        float: left;
        background: #2c3e50;
        color: #fff;
        height: 100vh;
    }
    
    .main {
        width: calc(100% - 70px);
        float: right;
    }

    .main-contact {
        height: 100px;
        line-height: 70px;
        background: #fff;
        padding: 10px;
    }

    .contact-name {
        margin-left: 20px;
        font-size: 20px;
    }
    
    .contact-img {
        width: 50px;
        height: 50px;
        border-radius: 50%;
        background: #fff;
        border: 3px solid #ccc;
    }

    .contact-img-large {
        height: 80px;
        width: 80px;
    }

    img.status-online {
        border: 3px solid #44e39d;
    }

    img.status-offline {
        border: 3px solid #ccc;
    }

    .main-chat {
        background: #e6eaea;
        height: calc(100vh - 100px);
    }

    ul.contacts {
        list-style: none;
        margin:0;
        padding: 10px 0 0 0;
    }

    ul.contacts li {
        text-align: center;
        padding: 10px 15px 15px 10px;
    }

    ul.contacts li:hover {
      cursor: pointer;
      background: #263543;
    }

    ul.contacts li.active {
      background: #263543;
    }
</style>
