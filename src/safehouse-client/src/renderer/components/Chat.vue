<template>
    <div class="main-container">
        <div class="side-bar">
            <contacts-bar></contacts-bar>
        </div>

        <active-chat></active-chat>
    </div>
</template>

<script>
  import ActiveChat from './chat/ActiveChat.vue'
  import ContactsBar from './contacts/ContactsBar.vue'

  export default {
    name: 'chat',

    components: {
      ActiveChat,
      ContactsBar
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
