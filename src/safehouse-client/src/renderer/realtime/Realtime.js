import Auth from '../api/auth/Auth'
import Message from './Message'
import Store from '../store'

var connection = {
  socket: null,
  server: null
}

var connectionReady = (event) => {
  console.log('Safehouse-Realtime - Ready for messages')

  sendMessage(new Message(1, Auth.getAuthToken()))
}

var handleMessage = (event) => {
  getBuffer(event.data, (buffer) => {
    var msg = Message.decode(buffer)
    console.log('msg type: ' + msg.type + ', payload: ' + msg.payload)

    if (msg.type === 2) {
      Store.commit('updateContactStatus', JSON.parse(msg.payload))
    }
  })
}

var sendMessage = (message) => {
  if (connection.socket.readyState === 1) {
    // todo: expandable buffer..
    connection.socket.send(message.encode(new ArrayBuffer(1024)))
  }
}

var disconnect = () => {
  connection.socket.close()
}

function getBuffer (blob, consumer) {
  var fileReader = new FileReader()

  fileReader.onload = function () {
    consumer(this.result)
  }

  fileReader.readAsArrayBuffer(blob)
}

export default {
  connect (server) {
    connection.server = server
    connection.socket = new WebSocket('ws://' + server.host + ':' + server.port)

    connection.socket.onmessage = handleMessage
    connection.socket.onopen = (e) => {
      connectionReady(e)
      server.ready()
    }
  },

  send: sendMessage,

  disconnect: disconnect,
  Message: Message
}
