<template>
  <div class="container">
    <h2>Safehouse Alpha</h2>
    <form>
      <div class="form-group">
        <label for="username">Username</label>
        <input type="username" id="email" class="form-control" v-model="username" placeholder="Username">
        <small class="form-text text-muted">The username you use to login to Safehouse.</small>
      </div>
      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" class="form-control" id="password" v-model="password" placeholder="Password">
        <small class="form-text text-muted">Always make sure your password is secure!</small>
      </div>

      <button type="submit" class="btn btn-primary" v-on:click="login">Login</button>
    </form>
  </div>
</template>

<script>
  import Realtime from '../realtime/Realtime'
  import ApiClient from '../api/Client'
  import Auth from '../api/auth/Auth'
  var ipc = require('electron').ipcRenderer

  function onLogin (router, store) {
    ipc.send('resize-window', { height: 720, width: 1280 })

    Realtime.connect({
      host: 'localhost',
      port: 1338
    })

    ApiClient.user.contacts().then((res) => {
      store.commit('setContacts', {
        contacts: res.data
      })

      router.push('chat')
    })
  }

  export default {
    methods: {
      login () {
        event.preventDefault()

        ApiClient.auth.authorise(this.username, this.password).then((res) => {
          Auth.setAuthentication(res.data.token)

          onLogin(this.$router, this.$store)
        }).catch((err) => {
          console.log(err)
        })
      }
    },

    data () {
      // if (Auth.isAuthenticated) {
      //   onLogin(this.$router)
      // }

      return { username: '', password: '' }
    },

    name: 'login'
  }
</script>

<style>
  .main-container {
    width: 100%;
    height: 100vh;
  }
</style>
