<template>
  <div class="add-contact" >
    <!-- Using modifiers -->
    <b-btn v-b-modal.addContact><i class="fas fa-user-plus"></i></b-btn>
    <!-- the modal -->
    <b-modal id="addContact" title="Add Contact" cancel-hidden="true" ok-hidden="true">
      <input class="form-control" placeholder="Username" />

      <template slot="modal-footer">
        <button class="btn btn-primary">Add Contact</button>
      </template>
    </b-modal>
  </div>
</template>

<script>
import Modal from 'bootstrap-vue/es/components/modal/modal'

export default {
  name: 'add-contact',
  components: {
    Modal
  }
}
</script>

<style>
  .add-contact {
    position: absolute;
    bottom: 0;
    left: 11px; 
    bottom: 12px;
    z-index: 99999999;
    text-align: center;
  }

  #addContact {
    color: #444;
    text-align: left;
  }
</style>