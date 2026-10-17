<script setup lang="ts">
import { ref,onMounted } from "vue";
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { z } from "zod";
import { Store } from "@tauri-apps/plugin-store";
import { useDutyTableStore } from "../piniastores/dutyTableStore";
const dutyTableStore = useDutyTableStore();
import check from "../scripts/check";

const schema = z.object({
    role_types: z.array(z.string()),
    groups: z.array(
        z.object({
            id: z.number(),
            assignments: z.array(z.array(z.string())),
        }),
    ),
});

type User = z.infer<typeof schema>;
const error = ref("");
function onFile(e: Event) {
    let data = null as null | User;
    error.value = "";
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    if (!file.name.toLowerCase().endsWith(".json")) {
        error.value = "请选择 .json 文件";
        input.value = "";
        return;
    }
    const reader = new FileReader();
    reader.onload = async (ev) => {
        let result = null;
        try {
            result = check(ev.target?.result as string);
        } catch (_e) {
            error.value = "JSON格式错误";
            return;
        }
        data = result;
        const store = await Store.load("settings.json");
        await store.set("table", data);
        dutyTableStore.$state.table = data;
    };
    reader.readAsText(file, "UTF-8");
    input.value = "";
}
const totalDaysUp = async () => {
    if (dutyTableStore.table != null) {
        if (dutyTableStore.totalDays + 1 < dutyTableStore.table.groups.length) {
            dutyTableStore.$state.totalDays++;
            const store = await Store.load("settings.json");
            await store.set("totalDays", dutyTableStore.totalDays);
        }
    }
};
const totalDaysDown = async () => {
    if (dutyTableStore.table != null) {
        if (dutyTableStore.totalDays - 1 >= 0) {
            dutyTableStore.$state.totalDays--;
            const store = await Store.load("settings.json");
            await store.set("totalDays", dutyTableStore.totalDays);
        }
    }
};
const autoStartStatus = ref(false)
onMounted(async() => {
  autoStartStatus.value = await isEnabled()
})
const toggleAutoStart = async () => {
  autoStartStatus.value = !autoStartStatus.value
  if (autoStartStatus.value) {
    await enable();
    return
  }
  await disable();
}
</script>

<template>
    <div class="flex-row flex gap-2">
        <button
            class="btn btn-ghost btn-circle btn-sm"
            @click="async()=>await dutyTableStore.flash()"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="size-5 text-gray-500 dark:text-gray-50"
            >
                <path
                    fill-rule="evenodd"
                    d="M4.755 10.059a7.5 7.5 0 0 1 12.548-3.364l1.903 1.903h-3.183a.75.75 0 1 0 0 1.5h4.992a.75.75 0 0 0 .75-.75V4.356a.75.75 0 0 0-1.5 0v3.18l-1.9-1.9A9 9 0 0 0 3.306 9.67a.75.75 0 1 0 1.45.388Zm15.408 3.352a.75.75 0 0 0-.919.53 7.5 7.5 0 0 1-12.548 3.364l-1.902-1.903h3.183a.75.75 0 0 0 0-1.5H2.984a.75.75 0 0 0-.75.75v4.992a.75.75 0 0 0 1.5 0v-3.18l1.9 1.9a9 9 0 0 0 15.059-4.035.75.75 0 0 0-.53-.918Z"
                    clip-rule="evenodd"
                />
            </svg>
        </button>
        <button
            class="btn btn-ghost btn-circle btn-sm"
            onclick="modal_1.showModal()"
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="size-5 text-gray-500 dark:text-gray-50"
            >
                <path
                    fill-rule="evenodd"
                    d="M11.078 2.25c-.917 0-1.699.663-1.85 1.567L9.05 4.889c-.02.12-.115.26-.297.348a7.493 7.493 0 0 0-.986.57c-.166.115-.334.126-.45.083L6.3 5.508a1.875 1.875 0 0 0-2.282.819l-.922 1.597a1.875 1.875 0 0 0 .432 2.385l.84.692c.095.078.17.229.154.43a7.598 7.598 0 0 0 0 1.139c.015.2-.059.352-.153.43l-.841.692a1.875 1.875 0 0 0-.432 2.385l.922 1.597a1.875 1.875 0 0 0 2.282.818l1.019-.382c.115-.043.283-.031.45.082.312.214.641.405.985.57.182.088.277.228.297.35l.178 1.071c.151.904.933 1.567 1.85 1.567h1.844c.916 0 1.699-.663 1.85-1.567l.178-1.072c.02-.12.114-.26.297-.349.344-.165.673-.356.985-.57.167-.114.335-.125.45-.082l1.02.382a1.875 1.875 0 0 0 2.28-.819l.923-1.597a1.875 1.875 0 0 0-.432-2.385l-.84-.692c-.095-.078-.17-.229-.154-.43a7.614 7.614 0 0 0 0-1.139c-.016-.2.059-.352.153-.43l.84-.692c.708-.582.891-1.59.433-2.385l-.922-1.597a1.875 1.875 0 0 0-2.282-.818l-1.02.382c-.114.043-.282.031-.449-.083a7.49 7.49 0 0 0-.985-.57c-.183-.087-.277-.227-.297-.348l-.179-1.072a1.875 1.875 0 0 0-1.85-1.567h-1.843ZM12 15.75a3.75 3.75 0 1 0 0-7.5 3.75 3.75 0 0 0 0 7.5Z"
                    clip-rule="evenodd"
                />
            </svg>
        </button>
        <dialog id="modal_1" class="modal">
            <div class="modal-box">
                <h3 class="text-lg font-bold">设置</h3>
                <p class="mb-1 mt-2 text-gray-500 dark:text-gray-200 text-sm">
                    值日表导入
                </p>
                <p v-if="error" class="text-red-500 text-sm">{{ error }}</p>
                <input
                    accept=".json,application/json"
                    @change="onFile"
                    type="file"
                    class="file-input file-input-sm"
                />
                <p class="mt-2 mb-1 text-gray-500 dark:text-gray-200 text-sm">
                    总天数设置
                </p>
                <div class="flex flex-row items-center gap-1">
                    <button class="btn btn-circle btn-sm" @click="totalDaysUp">
                        +
                    </button>
                    <div class="p-2">{{ dutyTableStore.totalDays }}</div>
                    <button
                        class="btn btn-circle btn-sm"
                        @click="totalDaysDown"
                    >
                        -
                    </button>
                </div>
                <p class="mb-1 mt-2 text-gray-500 dark:text-gray-200 text-sm">
                                   开机自启动
                               </p>
                <input type="checkbox" @click="toggleAutoStart" :checked="autoStartStatus" class="toggle" />
                <div class="modal-action">
                    <form method="dialog">
                        <button class="btn">关闭</button>
                    </form>
                </div>
            </div>
        </dialog>
        <button
            class="btn btn-ghost btn-circle btn-sm"
            @click="dutyTableStore.isToday=!dutyTableStore.isToday"
        ><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-5 inline text-gray-500">
          <path d="M5.055 7.06C3.805 6.347 2.25 7.25 2.25 8.69v8.122c0 1.44 1.555 2.343 2.805 1.628L12 14.471v2.34c0 1.44 1.555 2.343 2.805 1.628l7.108-4.061c1.26-.72 1.26-2.536 0-3.256l-7.108-4.061C13.555 6.346 12 7.249 12 8.689v2.34L5.055 7.061Z" />
        </svg>
</button>
    </div>
</template>
