const state = {
  contacts: {},
  activeContact: null
}

const mutations = {
  setContacts (state, payload) {
    state.contacts = {}

    for (var i = 0; i < payload.length; i++) {
      let contact = payload[i]

      state.contacts[contact.id] = contact
    }
  },

  setActiveContact (state, payload) {
    if (state.activeContact !== null && state.activeContact.id === payload.activeContact.id) {
      state.activeContact = null
    } else {
      state.activeContact = payload.activeContact
    }
  },

  updateContactStatus (state, payload) {
    var updateStatus = (contact) => {
      state.contacts[contact.id].status = contact.status.toLowerCase()

      if (state.activeContact.id === contact.id) {
        state.activeContact.status = contact.status.toLowerCase()
      }
    }

    if (payload instanceof Array) {
      for (var i = 0; i < payload.length; i++) {
        updateStatus(payload[i])
      }
    } else {
      updateStatus(payload)
    }

    console.log(state)
  }
}

export default {
  state,
  mutations
}
