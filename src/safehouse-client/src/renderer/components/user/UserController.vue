<template>
  <div class="user-ctrl">
    {{ user.username }}
    <div class="user-ctrl-expand">
      <i class="fas fa-sort-down"></i>
    </div>

  </div>
</template>

<script>
import { mapState } from 'vuex'

export default {
  name: 'user-controller',

  computed: {
    ...mapState({
      user: state => state.User.data
    })
  }
}
</script>

<style>
  .user-ctrl {
    background: #617487;  
    border-radius: 3px;
    position:absolute;
    color: #fff;
    z-index: 9999;
    top: 10px;
    right: 10px;
    padding: 5px 5px 5px 10px;
    min-width: 100px;
  }

  .user-ctrl:hover {
    cursor: pointer;
    background: #7b91a8;
  }

  .user-ctrl-expand {
    float: right;
    padding-right: 10px;
    position: absolute;
    top: 0;
    
    right: 0;
  }

</style>