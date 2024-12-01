<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const rules = [
  (v: any) =>
    (!!v && v.length > 0) ||
    "Please select a zip file containing codices exported from Novelcrafter.",
  (v: any) =>
    (v[0] && v[0].size > 0) ||
    "Please select a zip file containing codices exported from Novelcrafter.",
  (v: any) =>
    (v[0] && v[0].name.endsWith(".zip")) || "You can only select a zip file.",
];

const file: Ref<File | null> = ref(null);
const form: Ref<any> = ref(null);

async function createDatabase() {
  const formElement = form.value;
  const { valid } = await formElement.validate();

  if (valid) {
    const buffer = await file.value?.arrayBuffer();
    if (buffer) {
      const data = new Uint8Array(buffer);
      await invoke("load", { bytes: data });
    }
  }
}

async function launchClaude() {
  await invoke("launch");
}
</script>

<template>
  <v-layout row wrap class="rounded rounded-md">
    <v-container grid-list-md>
      <v-card>
        <v-card-title class="d-flex align-center justify-center">
          <strong>Novelcrafter MCP (Unofficial)</strong>
        </v-card-title>
        <v-card-subtitle>
          <a
            href="https://app.novelcrafter.com/"
            class="logo-area"
            target="_blank"
          >
            <img src="./assets/novelcrafter.svg" alt="Novelcrafter logo" />
          </a>
        </v-card-subtitle>
        <v-card-text>
          <v-sheet class="mx-auto" width="750">
            <v-container>
              <v-form
                @submit.prevent="createDatabase"
                validate-on="submit lazy"
                ref="form"
              >
                <v-row>
                  <v-col cols="5">
                    <v-label class="col">
                      <strong>
                        Select Novelcrafter codex export (ZIP file):
                      </strong>
                    </v-label>
                  </v-col>
                  <v-col cols="7">
                    <v-file-input
                      v-model="file"
                      variant="underlined"
                      required
                      :rules="rules"
                      label="File"
                      accept="application/zip"
                      density="compact"
                    ></v-file-input>
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="6">
                    <v-btn class="mt-2" type="submit" block
                      >Create SQLite Database</v-btn
                    >
                  </v-col>
                  <v-col cols="6">
                    <v-btn class="mt-2" block @click="launchClaude"
                      >Launch Claude Desktop</v-btn
                    >
                  </v-col>
                </v-row>
              </v-form>
            </v-container>
          </v-sheet>
        </v-card-text>
      </v-card>
    </v-container>
  </v-layout>
</template>

<style scoped>
.logo-area {
  width: 361px;
  height: 114px;
  display: block;
  margin: 0.5em auto;
}

.col {
  padding-top: 1em;
  padding-bottom: 1em;
}
</style>
