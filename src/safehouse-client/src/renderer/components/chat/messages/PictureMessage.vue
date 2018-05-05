<template>
  <div class='picture-msg'>
    <b-btn v-b-modal.drawMsg><i class='far fa-edit'></i></b-btn>
    
    <b-modal width='350px' ref="drawMsg" id='drawMsg' title='Draw Message' cancel-hidden='true' ok-hidden='true'>
      <canvas id='drawing-canvas' height='420px' width='420px'></canvas>

      <template slot="modal-footer">
        <button class="btn btn-primary" v-on:click="sendPicture">Send Picture</button>
      </template>
    </b-modal>
  </div>
</template>

<script>
  import { mapState } from 'vuex'
  import Modal from 'bootstrap-vue/es/components/modal/modal'
  import Auth from '../../../api/auth/Auth'
  import Realtime from '../../../realtime/Realtime'

  export default {
    name: 'picture-message',

    methods: {
      sendPicture () {
        var canvas = document.getElementById('drawing-canvas')

        let msg = {
          local: true,
          sender: Auth.userId(),
          user_id: this.activeContact.id,
          message: {
            type: 'img',
            msg: canvas.toDataURL()
          }
        }

        this.$refs.drawMsg.hide()
        this.$store.commit('newChatMessage', msg)
        Realtime.sendChatMessage(msg)
      }
    },

    mounted () {
      var canvas
      var ctx
      var flag = false
      var prevX = 0
      var currX = 0
      var prevY = 0
      var currY = 0
      var dotFlat = false

      var x = 'black'
      var y = 2

      canvas = document.getElementById('drawing-canvas')
      ctx = canvas.getContext('2d')
      ctx.fillStyle = 'white'
      ctx.fillRect(0, 0, canvas.width, canvas.height)

      canvas.addEventListener('mousemove', function (e) {
        console.log(e)
        findxy('move', e)
      }, false)

      canvas.addEventListener('mousedown', function (e) {
        findxy('down', e)
      }, false)

      canvas.addEventListener('mouseup', function (e) {
        findxy('up', e)
      }, false)

      canvas.addEventListener('mouseout', function (e) {
        findxy('out', e)
      }, false)

      canvas.addEventListener('touchmove', function (e) {
        console.log(e)
        findxy('move', e.touches[0])
      }, false)

      canvas.addEventListener('touchstart', function (e) {
        findxy('down', e.touches[0])
      }, false)

      canvas.addEventListener('touchend', function (e) {
        findxy('up', e.touches[0])
      }, false)

      function draw () {
        ctx.beginPath()
        ctx.moveTo(prevX, prevY)
        ctx.lineTo(currX, currY)
        ctx.strokeStyle = x
        ctx.lineWidth = y
        ctx.stroke()
        ctx.closePath()
      }

      function findxy (res, e) {
        if (res === 'down') {
          prevX = currX
          prevY = currY
          currX = e.clientX - canvas.getBoundingClientRect().left
          currY = e.clientY - canvas.getBoundingClientRect().top

          flag = true
          dotFlat = true
          if (dotFlat) {
            ctx.beginPath()
            ctx.fillStyle = x
            ctx.fillRect(currX, currY, 2, 2)
            ctx.closePath()
            dotFlat = false
          }
        }

        if (res === 'up' || res === 'out') {
          flag = false
        }

        if (res === 'move') {
          if (flag) {
            prevX = currX
            prevY = currY
            currX = e.clientX - canvas.getBoundingClientRect().left
            currY = e.clientY - canvas.getBoundingClientRect().top
            draw()
          }
        }
      }
    },

    computed: {
      ...mapState({
        activeContact: state => state.User.activeContact
      })
    },

    components: {
      Modal
    }
}
</script>

<style>
  .picture-msg {
    display: inline-block;
    padding-left: 10px;
    text-align:center;
  }

  canvas {
    display: inline;
    border: 2px solid #f1f1f1;
    border-radius: 3px;
  }
</style>