<script setup lang="ts">
import { ref } from "vue"
import { VirtualMachine } from "brainfuck-rs"

let programCode = ref("")
let inputText = ref("")
let outputText = ref("")
let errorMsg = ref("")
let startAddress = ref(0)

function execute() {
  const brainfuck = VirtualMachine.new()

  brainfuck.set_address_pointer_js(startAddress.value);
  brainfuck.load_js(programCode.value)
  brainfuck.input_js(inputText.value + "\n");
  try {
    const output = brainfuck.run_js()
    outputText.value = output
  } catch (error: any) {
    errorMsg.value = error.message
  }
}
</script>

<template>
  <v-container>
    <v-row>
      <v-col cols="6">
        <v-textarea
          label="Program"
          v-model="programCode"
        ></v-textarea>
      </v-col>
      <v-col cols="6">
        <v-textarea
          label="Output"
          v-model="outputText"
          readonly
        ></v-textarea>
      </v-col>
    </v-row>
    <v-row>
      <v-col cols="6">
        <v-text-field
        label="Input"
        v-model="inputText"
      ></v-text-field>
      </v-col>
      <v-col cols="5">
        <div class="text-caption">Starting address</div>
        <v-slider
          v-model="startAddress"
          min="0"
          max="65535"
          step="4096"
          track-color="grey"
          thumb-size="15"
          thumb-label
        >
          <template v-slot:append>
            <v-text-field
              v-model.number="startAddress"
              type="number"
              style="width: 110px"
            ></v-text-field>
          </template>
        </v-slider>
      </v-col>
      <v-col cols="1">
        <v-btn
          @click="execute"
          class="mt-8"
        >Execute</v-btn>
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-alert
          :style="{visibility : errorMsg ? 'visible' : 'hidden'}"
          type="error"
          variant="outlined"
          density="compact"
        >{{ errorMsg }}</v-alert>
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
</style>
