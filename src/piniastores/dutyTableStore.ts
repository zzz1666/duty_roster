import { defineStore } from "pinia";
import { Store } from "@tauri-apps/plugin-store";
import check from "../scripts/check";

export const useDutyTableStore = defineStore("dutyTableStore", {
  state: () => {
    return { table:null as any, totalDays:0 as any,dayNumber: 0,isToday:true };
  },
  actions: {
    async flash() {
      const store = await Store.load("settings.json");
      const totalDays = await store.get("totalDays");
      const table = await store.get("table");
      try {
        if (table) {
          if (typeof table == 'object') {
            check(table)
            this.table = table
          }
        }
      } catch (e) {}
      this.totalDays = totalDays || 0
      this.dayNumber = Math.floor((Date.now() - new Date().getTimezoneOffset() * 60000) / 86400000) + 1
    }
  }
});
