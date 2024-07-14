<script setup lang="ts">
  import { sub } from 'date-fns';
  import type { Period, Range } from '~/types';
  import type { Avatar } from '#ui/types';
  const people = [
    {
      id: 'benjamincanac',
      label: 'benjamincanac',
      href: 'https://github.com/benjamincanac',
      target: '_blank',
      avatar: { src: 'https://avatars.githubusercontent.com/u/739984?v=4' },
    },
    {
      id: 'Atinux',
      label: 'Atinux',
      href: 'https://github.com/Atinux',
      target: '_blank',
      avatar: { src: 'https://avatars.githubusercontent.com/u/904724?v=4' },
    },
    {
      id: 'smarroufin',
      label: 'smarroufin',
      href: 'https://github.com/smarroufin',
      target: '_blank',
      avatar: { src: 'https://avatars.githubusercontent.com/u/7547335?v=4' },
    },
    {
      id: 'nobody',
      label: 'Nobody',
      icon: 'i-heroicons-user-circle',
    },
  ];
  const selected = ref(people[0]);
  const range = ref<Range>({ start: sub(new Date(), { days: 14 }), end: new Date() });
  const period = ref<Period>('daily');
</script>

<template>
  <UDashboardPage>
    <UDashboardPanel grow>
      <UDashboardNavbar title="Log Playback Viewer">
        <template #right>
          <p class="mt-2">Version: 1.0.0</p>

          <!-- <UDropdown :items="items">
            <UButton icon="i-heroicons-plus" size="md" class="ml-1.5 rounded-full" />
          </UDropdown> -->
        </template>
      </UDashboardNavbar>

      <UDashboardToolbar>
        <template #left>
          <PlaybackDateRangePicker v-model="range" class="-ml-2.5" />
          <PlaybackPeriodSelect v-model="period" :range="range" />
        </template>
      </UDashboardToolbar>
      <UDashboardToolbar>
        <template #left>
          <USelectMenu v-model="selected" :options="people">
            <template #leading>
              <UIcon v-if="selected.icon" :name="(selected.icon as string)" class="w-5 h-5" />
              <UAvatar v-else-if="selected.avatar" v-bind="(selected.avatar as Avatar)" size="2xs" />
            </template>
          </USelectMenu>
          <USelectMenu v-model="selected" :options="people">
            <template #leading>
              <UIcon v-if="selected.icon" :name="(selected.icon as string)" class="w-5 h-5" />
              <UAvatar v-else-if="selected.avatar" v-bind="(selected.avatar as Avatar)" size="2xs" />
            </template>
          </USelectMenu>
          <USelectMenu v-model="selected" :options="people">
            <template #leading>
              <UIcon v-if="selected.icon" :name="(selected.icon as string)" class="w-5 h-5" />
              <UAvatar v-else-if="selected.avatar" v-bind="(selected.avatar as Avatar)" size="2xs" />
            </template>
          </USelectMenu>
        </template>

        <template #right>
          <USelectMenu label="Display" icon="i-heroicons-computer-desktop" />
          <USelectMenu label="Display" icon="i-heroicons-computer-desktop" />
        </template>
      </UDashboardToolbar>

      <UDashboardPanelContent>
        <div class="grid lg:grid-cols-4 lg:items-start gap-8 mt-8 mb-3">
          <CoreDataCard v-for="i in 8" />
        </div>
        <PlaybackChart :period="period" :range="range" />
      </UDashboardPanelContent>
    </UDashboardPanel>
  </UDashboardPage>
</template>
