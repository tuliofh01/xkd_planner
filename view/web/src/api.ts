import { invoke } from "@tauri-apps/api/core";

/**
 * Typed frontend wrappers for Tauri commands.
 *
 * Interfaces mirror Rust records in `model/src/records` and collections.
 * Keep shapes in sync with Rust/serde output.
 */

// HINT: these interfaces mirror the `pub struct` definitions in
// crates/models. Keeping them in sync by hand is fine for a small MVP;
// later you could generate them from Rust with ts-rs or serde-generate.
/** Task record returned by Tauri commands. */
export interface Task {
  id: string;
  user_id: string;
  title: string;
  description: string;
  due_at: string | null;
  status: string;
  created_at: string;
}

/** User record returned by Tauri commands. */
export interface User {
  id: string;
  username: string;
  password: string;
  status: boolean;
  tasks: Task[];
}

/** Activity/log record. */
export interface SlogRecord {
  id: string;
  user_id: string;
  task_id: string | null;
  message: string;
  created_at: string;
}

/** Pomodoro/timer session record. */
export interface PtimerSession {
  id: string;
  user_id: string;
  task_id: string;
  started_at: string;
  ended_at: string | null;
}

/** Timeline entry record. */
export interface TimelineEntry {
  id: string;
  user_id: string;
  task_id: string;
  starts_at: string;
  ends_at: string | null;
}

// HINT: Tauri converts camelCase argument keys to the snake_case names used
// by Rust commands, so `userId` here matches `user_id` in lib.rs.
//
// HINT: every command body is currently `todo!()` in Rust. Calls will build
// and type-check, but they panic at runtime until you implement the commands.
/** Tauri invoke API wrappers used by the UI. */
export const api = {
  register: (username: string, password: string) =>
    invoke<User>("register", { username, password }),
  login: (username: string, password: string) => invoke<User>("login", { username, password }),
  listUsers: () => invoke<User[]>("list_users"),
  listTasks: (userId: string) => invoke<Task[]>("list_tasks", { userId }),
  createTask: (userId: string, title: string, description: string, dueAt: string | null) =>
    invoke<Task>("create_task", { userId, title, description, dueAt }),
  completeTask: (id: string) => invoke<Task>("complete_task", { id }),
  deleteTask: (id: string) => invoke<void>("delete_task", { id }),
  listLogs: (userId: string) => invoke<SlogRecord[]>("list_logs", { userId }),
  writeLog: (userId: string, taskId: string | null, message: string) =>
    invoke<SlogRecord>("write_log", { userId, taskId, message }),
  startTimer: (userId: string, taskId: string) =>
    invoke<PtimerSession>("start_timer", { userId, taskId }),
  stopTimer: (id: string) => invoke<PtimerSession>("stop_timer", { id }),
  listTimers: (userId: string) => invoke<PtimerSession[]>("list_timers", { userId }),
  addTimelineEntry: (userId: string, taskId: string, startsAt: string, endsAt: string | null) =>
    invoke<TimelineEntry>("add_timeline_entry", { userId, taskId, startsAt, endsAt }),
  listTimeline: (userId: string) => invoke<TimelineEntry[]>("list_timeline", { userId }),
};
