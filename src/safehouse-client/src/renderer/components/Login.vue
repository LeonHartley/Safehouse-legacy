<template>
  <div class="container">
    <h2>Safehouse Alpha</h2>
    <form>
      <div class="form-group">
        <label for="username">Username</label>
        <input type="username" id="email" class="form-control" placeholder="Username">
        <small class="form-text text-muted">The username you use to login to Safehouse.</small>
      </div>
      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" class="form-control" id="password" placeholder="Password">
        <small class="form-text text-muted">Always make sure your password is secure!</small>
      </div>

      <button type="submit" class="btn btn-primary" v-on:click="test">Login</button>
    </form>
  </div>
</template>

<script>
  import Realtime from '../realtime/Realtime'
  import ApiClient from '../api/Client'
 
  var ipc = require('electron').ipcRenderer

  export default {
    methods: {
      test () {
        Realtime.send({msg: 'Hiiii'})
      }
    },

    data () {
 //     ipc.send('resize-window', { height: 450, width: 300 })
      return {}
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
