<script setup lang="ts">
import { ref } from 'vue'
import Drawer from 'primevue/drawer'
import SelectButton from 'primevue/selectbutton'
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import { useSettings, type ThemeMode, type ViewMode, type SortMode, type PullMode, type CloseAction } from '@/composables/useSettings'
import { useProjectsStore } from '@/stores/projects'
import { useRepoStore } from '@/stores/repo'
import { open } from '@tauri-apps/plugin-dialog'
import {
  Palette,
  GitBranch,
  Bell,
  Wrench,
  Database,
  TriangleAlert,
} from 'lucide-vue-next'

const visible = ref(false)
const { settings, update, resetAll } = useSettings()
const store = useProjectsStore()
const repo = useRepoStore()

const showResetConfirm = ref(false)
const showClearConfirm = ref(false)

// === 选项定义 ===
const themeOptions = [
  { label: '深色', value: 'dark' as ThemeMode },
  { label: '浅色', value: 'light' as ThemeMode },
  { label: '跟随系统', value: 'system' as ThemeMode },
]

const viewModeOptions = [
  { label: '卡片', value: 'card' as ViewMode },
  { label: '列表', value: 'list' as ViewMode },
]

const sortModeOptions = [
  { label: '默认', value: 'default' as SortMode },
  { label: '最近打开', value: 'recent' as SortMode },
  { label: '名称', value: 'name' as SortMode },
  { label: '自定义', value: 'custom' as SortMode },
]

const pullModeOptions = [
  { label: 'Merge', value: 'merge' as PullMode },
  { label: 'Rebase', value: 'rebase' as PullMode },
]

const closeActionOptions = [
  { label: '每次询问', value: 'ask' as CloseAction },
  { label: '最小化', value: 'minimize' as CloseAction },
  { label: '退出', value: 'exit' as CloseAction },
]

// === 操作 ===
function show() {
  visible.value = true
}

async function selectGitPath() {
  try {
    const selected = await open({
      multiple: false,
      title: '选择 Git 可执行文件',
      filters: [{ name: 'Git', extensions: ['exe'] }],
    })
    if (selected) {
      update('customGitPath', selected as string)
    }
  } catch {
    // 用户取消
  }
}

function handleReset() {
  resetAll()
  showResetConfirm.value = false
  repo.showToast('success', '已重置所有设置')
}

async function handleClearProjects() {
  const paths = store.projects.map(p => p.path)
  for (const p of paths) {
    await store.removeProject(p)
  }
  showClearConfirm.value = false
  repo.showToast('success', '已清空所有项目')
}

defineExpose({ show })
</script>

<template>
  <Drawer
    v-model:visible="visible"
    header="设置"
    position="right"
    :style="{ width: '420px' }"
    class="settings-drawer"
  >
    <div class="settings-content">
      <!-- 外观设置 -->
      <div class="settings-group">
        <div class="group-header">
          <Palette :size="16" />
          <span>外观</span>
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">主题模式</span>
            <span class="label-desc">选择界面配色方案</span>
          </div>
          <SelectButton
            :modelValue="settings.theme"
            @update:modelValue="(v: ThemeMode) => update('theme', v)"
            :options="themeOptions"
            optionLabel="label"
            optionValue="value"
            :allowEmpty="false"
            size="small"
          />
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">默认视图</span>
            <span class="label-desc">项目列表的展示方式</span>
          </div>
          <SelectButton
            :modelValue="settings.viewMode"
            @update:modelValue="(v: ViewMode) => update('viewMode', v)"
            :options="viewModeOptions"
            optionLabel="label"
            optionValue="value"
            :allowEmpty="false"
            size="small"
          />
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">默认排序</span>
            <span class="label-desc">项目列表的排序规则</span>
          </div>
          <SelectButton
            :modelValue="settings.sortMode"
            @update:modelValue="(v: SortMode) => update('sortMode', v)"
            :options="sortModeOptions"
            optionLabel="label"
            optionValue="value"
            :allowEmpty="false"
            size="small"
          />
        </div>
      </div>

      <!-- Git 设置 -->
      <div class="settings-group">
        <div class="group-header">
          <GitBranch :size="16" />
          <span>Git</span>
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">默认拉取方式</span>
            <span class="label-desc">执行 Pull 时的默认策略</span>
          </div>
          <SelectButton
            :modelValue="settings.pullMode"
            @update:modelValue="(v: PullMode) => update('pullMode', v)"
            :options="pullModeOptions"
            optionLabel="label"
            optionValue="value"
            :allowEmpty="false"
            size="small"
          />
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">提交历史条数</span>
            <span class="label-desc">加载的最大提交记录数量</span>
          </div>
          <InputNumber
            :modelValue="settings.commitHistoryCount"
            @update:modelValue="(v: number | null) => update('commitHistoryCount', v ?? 100)"
            :min="50"
            :max="500"
            :step="50"
            showButtons
            size="small"
            :style="{ width: '130px' }"
          />
        </div>
      </div>

      <!-- 通知与提示 -->
      <div class="settings-group">
        <div class="group-header">
          <Bell :size="16" />
          <span>通知与提示</span>
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">关闭窗口行为</span>
            <span class="label-desc">点击关闭按钮时的操作</span>
          </div>
          <SelectButton
            :modelValue="settings.closeAction"
            @update:modelValue="(v: CloseAction) => update('closeAction', v)"
            :options="closeActionOptions"
            optionLabel="label"
            optionValue="value"
            :allowEmpty="false"
            size="small"
          />
        </div>
      </div>

      <!-- 高级设置 -->
      <div class="settings-group">
        <div class="group-header">
          <Wrench :size="16" />
          <span>高级</span>
        </div>

        <div class="setting-item setting-item-vertical">
          <div class="setting-label">
            <span class="label-text">自定义 Git 路径</span>
            <span class="label-desc">指定 Git 可执行文件路径，留空使用系统 PATH</span>
          </div>
          <div class="git-path-row">
            <InputText
              :modelValue="settings.customGitPath"
              @update:modelValue="(v: string | undefined) => update('customGitPath', v ?? '')"
              placeholder="使用系统默认 Git"
              size="small"
              class="git-path-input"
            />
            <Button size="small" severity="secondary" variant="outlined" @click="selectGitPath">浏览</Button>
            <Button
              v-if="settings.customGitPath"
              size="small"
              severity="secondary"
              variant="text"
              @click="update('customGitPath', '')"
              title="清除"
            >清除</Button>
          </div>
        </div>
      </div>

      <!-- 数据管理 -->
      <div class="settings-group">
        <div class="group-header">
          <Database :size="16" />
          <span>数据管理</span>
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">重置所有设置</span>
            <span class="label-desc">恢复到默认设置</span>
          </div>
          <Button
            v-if="!showResetConfirm"
            size="small"
            severity="secondary"
            variant="outlined"
            @click="showResetConfirm = true"
          >重置</Button>
          <div v-else class="confirm-actions">
            <Button size="small" severity="danger" @click="handleReset">确认重置</Button>
            <Button size="small" severity="secondary" variant="text" @click="showResetConfirm = false">取消</Button>
          </div>
        </div>

        <div class="setting-item">
          <div class="setting-label">
            <span class="label-text">清空项目列表</span>
            <span class="label-desc">移除所有已添加的项目</span>
          </div>
          <Button
            v-if="!showClearConfirm"
            size="small"
            severity="danger"
            variant="outlined"
            @click="showClearConfirm = true"
          >清空</Button>
          <div v-else class="confirm-actions">
            <Button size="small" severity="danger" @click="handleClearProjects">
              <TriangleAlert :size="14" class="mr-1" />
              确认清空
            </Button>
            <Button size="small" severity="secondary" variant="text" @click="showClearConfirm = false">取消</Button>
          </div>
        </div>
      </div>

      <!-- 版本信息 -->
      <div class="settings-footer">
        <span class="version-text">GitVista v0.5.0</span>
      </div>
    </div>
  </Drawer>
</template>

<style scoped>
.settings-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding-bottom: 16px;
}

.settings-group {
  background: var(--p-content-hover-background);
  border: 1px solid var(--p-content-border-color);
  border-radius: var(--p-border-radius);
  overflow: hidden;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  font-size: 13px;
  font-weight: 600;
  color: var(--p-text-color);
  border-bottom: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--p-content-border-color);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item-vertical {
  flex-direction: column;
  align-items: stretch;
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.label-text {
  font-size: 13px;
  font-weight: 500;
  color: var(--p-text-color);
}

.label-desc {
  font-size: 11px;
  color: var(--p-text-muted-color);
}

.git-path-row {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}

.git-path-input {
  flex: 1;
  min-width: 0;
}

.confirm-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}

.mr-1 {
  margin-right: 4px;
}

.settings-footer {
  text-align: center;
  padding: 8px 0;
}

.version-text {
  font-size: 11px;
  color: var(--p-text-muted-color);
}
</style>
