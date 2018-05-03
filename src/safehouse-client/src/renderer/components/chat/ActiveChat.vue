<template>
  <div class="main" v-if="activeContact != null">
    <div class="main-contact" >
      <img :src="activeContact.avatar" :class="{
        'contact-img': true,
        'contact-img-large': true,
        'status-online': activeContact.status == 'online',
        'status-offline': activeContact == null || activeContact.status == 'offline'
      }" />
      
      <span class="contact-name">{{ activeContact.username }}</span>
    </div>
    <div class="main-chat" v-chat-scroll="{always: false}">
      <div class="messages">
        <ul v-if="activeChat.messages.length > 0">
          <li v-for="(message, index) in activeChat.messages" :key="index" >
          <div :class="{
            'chat-message': true,
            'sent': message.local
          }">{{message.txt}}</div>
          <div class="clear"></div>
          </li>
        </ul>
      </div>

      <div class="message-box">
        <form>
          <textarea v-model="currentMessage" v-on:keyup.enter="sendMessage" class="form-control"></textarea>
        </form>
      </div>
    </div>
  </div>
  <div class="main main-none" v-else>
    <div class="none-selected"><span style="font-size: 300px;"><i  class="far fa-comments"></i></span> <br />Start a conversation</div>
  </div>
</template>

<script>
  import { mapState } from 'vuex'

  import Realtime from '../../realtime/Realtime'
  import Auth from '../../api/auth/Auth'

  export default {
    name: 'active-chat',
    methods: {
      sendMessage (e) {
        e.preventDefault()

        if (this.currentMessage.match(/^ *$/) !== null) {
          return
        }

        var msg = {
          local: true,
          sender: Auth.userId(),
          user_id: this.activeContact.id,
          message: this.currentMessage
        }

        Realtime.send(new Realtime.Message(3, JSON.stringify(msg)))
        this.$store.commit('newChatMessage', msg)

        this.currentMessage = ''
      }
    },

    data () {
      return {
        userId: Auth.userId(),
        currentMessage: ''
      }
    },

    computed: {
      ...mapState({
        activeChat: (state) => state.Chat.activeChat,
        chats: (state) => state.Chat.chats,
        activeContact: state => state.User.activeContact
      })
    }
  }
</script>

<style>
  .main {
    position: relative;
  }

  .message-box { 
    position: absolute;
    bottom: 10px;
    right: 0;
    left: 0;
  }

  .message-box textarea {
    width: 95%;
    display: block;
    margin: auto;
  }

  .main-chat {
    overflow-y: scroll;
  }

  .messages ul {
    list-style: none;
    width: 100%;
    padding: 0 0 80px 0;
  }

  .messages li {
    width: 100%;
    position: relative;
  }

  .chat-message {
    display: inline-block;
    background: #435f7a;
    color: #fff;
    padding: 10px;
    border-radius: 5px;
    margin: 10px 15px 0 15px;
  }

  .chat-message.sent {
    float: right;
    background: #f9f9f9;
    color: #555;
  }

  .clear {
    clear: both;
  }

  .main-none {
    background: #e6eaea;
    height: 100vh;
  }

  .none-selected {
    color: #f9f9f9;
    font-weight: bold;
    text-align: center;
    padding-top: 20px;
    font-size: 50px;
  }
</style>
