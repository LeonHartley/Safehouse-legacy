const state = {
  chats: {},
  activeChat: null
}

const mutations = {
  newChatMessage (state, payload) {
    var local = payload.local !== undefined
    var id = local ? payload.user_id : payload.sender

    if (state.chats[id] === undefined) {
      state.chats[id] = {
        user_id: id,
        messages: []
      }
    }

    var msg = {
      local: local,
      from: id,
      type: payload.message.type,
      txt: payload.message.type === 'img' ? '<img src="' + payload.message.msg + '" heigth="420px" width="420px" />' : payload.message.msg
    }

    state.chats[id].messages.push(msg)

    if (state.activeChat.user_id === id) {
      state.activeChat.messages.push(msg)
    }
  },

  startChat (state, payload) {
    if (state.chats[payload] === undefined) {
      state.chats[payload] = {
        user_id: payload,
        messages: []
      }
    }

    state.activeChat = state.chats[payload]
  }
}

export default {
  state,
  mutations
}
