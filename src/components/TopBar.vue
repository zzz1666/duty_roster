<script setup lang="ts">
const props = defineProps<{
  startDragging: Function,
  close:Function
}>()
const handleDragStart = async (e: MouseEvent | TouchEvent) => {
  if (e.type === 'touchstart') {
    e.preventDefault()
  }
  await props.startDragging()
}
</script>

<template>
<div class="fixed p-2 top-0 w-full">
    <div class="flex flex-row gap-1">
    <button class="btn btn-xs"
      @mousedown.left="handleDragStart"
      @touchstart.prevent="handleDragStart"
    ><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-5 inline text-gray-500 dark:text-gray-50">
      <path fill-rule="evenodd" d="M2.25 6a3 3 0 0 1 3-3h13.5a3 3 0 0 1 3 3v12a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3V6Zm18 3H3.75v9a1.5 1.5 0 0 0 1.5 1.5h13.5a1.5 1.5 0 0 0 1.5-1.5V9Zm-15-3.75A.75.75 0 0 0 4.5 6v.008c0 .414.336.75.75.75h.008a.75.75 0 0 0 .75-.75V6a.75.75 0 0 0-.75-.75H5.25Zm1.5.75a.75.75 0 0 1 .75-.75h.008a.75.75 0 0 1 .75.75v.008a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V6Zm3-.75A.75.75 0 0 0 9 6v.008c0 .414.336.75.75.75h.008a.75.75 0 0 0 .75-.75V6a.75.75 0 0 0-.75-.75H9.75Z" clip-rule="evenodd" />
    </svg>&nbsp;移动
</button>
<button @click="()=>close()" class="btn btn-xs btn-circle">
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-5 inline text-gray-500 dark:text-gray-50">
      <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25Zm-1.72 6.97a.75.75 0 1 0-1.06 1.06L10.94 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06L12 13.06l1.72 1.72a.75.75 0 1 0 1.06-1.06L13.06 12l1.72-1.72a.75.75 0 1 0-1.06-1.06L12 10.94l-1.72-1.72Z" clip-rule="evenodd" />
    </svg>
</button>
    </div>
</div>
</template>
