<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
const appWindow = getCurrentWindow()
import TopBar from './components/TopBar.vue';
import Settings from './components/Settings.vue';
import { onMounted } from 'vue';
import { useDutyTableStore } from './piniastores/dutyTableStore';
const dutyTableStore = useDutyTableStore();
onMounted(async () => {
  await dutyTableStore.flash()
})

const getNumToday = (plus1: boolean) => {
  let num = dutyTableStore.totalDays + (plus1 && 1) + dutyTableStore.dayNumber
  if (dutyTableStore.table) {
   return num % dutyTableStore.table.groups.length
  }
  return
}
</script>

<template>
<div class="bg-gray-50/95 gap-2 flex flex-col dark:bg-gray-900/95 rounded-sm w-full h-full overflow-hidden">
    <TopBar :close="appWindow.close" :startDragging="appWindow.startDragging"></TopBar>
    <div class="my-auto p-2 w-full mt-8 text-3xl">
        <div v-if="dutyTableStore.table" class="flex flex-col gap-1">
            <div class="border-gray-300 border-b-2">{{dutyTableStore.isToday?'今天':'明天'}}值日生 第{{dutyTableStore.table.groups[getNumToday(!dutyTableStore.isToday)||0].id||"?"}}组</div>
            <div class="border-gray-300 border-b-2" v-for="(value,i) in dutyTableStore.table.groups[getNumToday(!dutyTableStore.isToday)||0].assignments">
                {{dutyTableStore.table.role_types[i]||"?"}}:
                <div class="inline" v-for="value1 in value">
                    {{value1}}&nbsp;
                </div>
            </div>
        </div>
    </div>
    <div class="float-end p-2"><Settings></Settings></div>
</div>
</template>
