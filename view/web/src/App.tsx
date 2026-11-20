import { useCallback, useEffect, useState } from "react";
import {
  api,
  type PtimerSession,
  type SlogRecord,
  type Task,
  type TimelineEntry,
  type User,
} from "./api";

// HINT: one entry per MVP screen. Add a new screen by extending this list and
// rendering the matching component below — no router needed for the skeleton.
type Screen = "tasks" | "timer" | "timeline" | "logs";

const SCREENS: { id: Screen; label: string }[] = [
  { id: "tasks", label: "Daily plan" },
  { id: "timer", label: "Timer" },
  { id: "timeline", label: "Timeline" },
  { id: "logs", label: "Logs" },
];

/** Root React application shell. */
export default function App() {
  // HINT: `user` doubles as the auth state and the source of the user id that
  // every other command needs.
  const [user, setUser] = useState<User | null>(null);
  const [screen, setScreen] = useState<Screen>("tasks");
  const [tasks, setTasks] = useState<Task[]>([]);
  const [error, setError] = useState<string | null>(null);

  const loadTasks = useCallback(async (userId: string) => {
    try {
      setTasks(await api.listTasks(userId));
      setError(null);
    } catch (cause) {
      setError(String(cause));
    }
  }, []);

  useEffect(() => {
    if (user) {
      void loadTasks(user.id);
    }
  }, [user, loadTasks]);

  return (
    <main className="app">
      <header className="app__header">
        <h1>xkdPlanner</h1>
        <p>{user ? `Signed in as ${user.username}` : "Not signed in"}</p>
      </header>

      {error && <p className="app__error">{error}</p>}

      {!user ? (
        <AuthPanel onAuthenticated={setUser} onError={setError} />
      ) : (
        <>
          <nav className="app__tabs">
            {SCREENS.map((entry) => (
              <button
                key={entry.id}
                type="button"
                className={entry.id === screen ? "is-active" : ""}
                onClick={() => setScreen(entry.id)}
              >
                {entry.label}
              </button>
            ))}
          </nav>

          {screen === "tasks" && (
            <TaskPanel userId={user.id} tasks={tasks} onRefresh={() => loadTasks(user.id)} />
          )}
          {screen === "timer" && <TimerPanel userId={user.id} tasks={tasks} />}
          {screen === "timeline" && <TimelinePanel userId={user.id} />}
          {screen === "logs" && <LogsPanel userId={user.id} />}
        </>
      )}
    </main>
  );
}

// HINT: props are typed inline for brevity. In a larger app you would extract
// a `type Props = { ... }` alias per component.
function AuthPanel({
  onAuthenticated,
  onError,
}: {
  onAuthenticated: (user: User) => void;
  onError: (message: string) => void;
}) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  async function run(action: "login" | "register") {
    try {
      const user =
        action === "login"
          ? await api.login(username, password)
          : await api.register(username, password);
      onAuthenticated(user);
    } catch (cause) {
      onError(String(cause));
    }
  }

  return (
    <section className="panel">
      <h2>Sign in</h2>
      <label className="field">
        <span>Username</span>
        <input value={username} onChange={(event) => setUsername(event.target.value)} />
      </label>
      <label className="field">
        <span>Password</span>
        <input
          type="password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
      </label>
      <div className="panel__actions">
        <button type="button" onClick={() => void run("login")}>
          Log in
        </button>
        <button type="button" className="ghost" onClick={() => void run("register")}>
          Register
        </button>
      </div>
    </section>
  );
}

function TaskPanel({
  userId,
  tasks,
  onRefresh,
}: {
  userId: string;
  tasks: Task[];
  onRefresh: () => void;
}) {
  const [title, setTitle] = useState("");

  async function add() {
    if (!title.trim()) return;
    await api.createTask(userId, title, "", null);
    setTitle("");
    onRefresh();
  }

  async function complete(id: string) {
    await api.completeTask(id);
    onRefresh();
  }

  return (
    <section className="panel">
      <h2>Daily plan</h2>
      <ul className="list">
        {tasks.map((task) => (
          <li key={task.id}>
            <span className={task.status === "completed" ? "is-done" : ""}>{task.title}</span>
            {task.status !== "completed" && (
              <button type="button" className="ghost" onClick={() => void complete(task.id)}>
                Done
              </button>
            )}
          </li>
        ))}
        {tasks.length === 0 && <li className="is-empty">No tasks yet.</li>}
      </ul>
      <div className="panel__actions">
        <input
          value={title}
          placeholder="New task"
          onChange={(event) => setTitle(event.target.value)}
        />
        <button type="button" onClick={() => void add()}>
          Add
        </button>
      </div>
    </section>
  );
}

function TimerPanel({ userId, tasks }: { userId: string; tasks: Task[] }) {
  const [session, setSession] = useState<PtimerSession | null>(null);
  const [taskId, setTaskId] = useState("");

  async function start() {
    if (!taskId) return;
    setSession(await api.startTimer(userId, taskId));
  }

  async function stop() {
    if (!session) return;
    await api.stopTimer(session.id);
    setSession(null);
  }

  return (
    <section className="panel">
      <h2>Pomodoro timer</h2>
      <label className="field">
        <span>Task</span>
        <select value={taskId} onChange={(event) => setTaskId(event.target.value)}>
          <option value="">Select a task…</option>
          {tasks.map((task) => (
            <option key={task.id} value={task.id}>
              {task.title}
            </option>
          ))}
        </select>
      </label>
      <div className="panel__actions">
        <button type="button" disabled={!!session} onClick={() => void start()}>
          Start
        </button>
        <button type="button" disabled={!session} onClick={() => void stop()}>
          Stop
        </button>
      </div>
    </section>
  );
}

function TimelinePanel({ userId }: { userId: string }) {
  const [entries, setEntries] = useState<TimelineEntry[]>([]);

  useEffect(() => {
    void api
      .listTimeline(userId)
      .then(setEntries)
      .catch(() => setEntries([]));
  }, [userId]);

  return (
    <section className="panel">
      <h2>Timeline</h2>
      <ul className="list">
        {entries.map((entry) => (
          <li key={entry.id}>
            <span>{entry.starts_at}</span>
            <span className="muted">{entry.ends_at ?? "running"}</span>
          </li>
        ))}
        {entries.length === 0 && <li className="is-empty">Nothing scheduled.</li>}
      </ul>
    </section>
  );
}

function LogsPanel({ userId }: { userId: string }) {
  const [logs, setLogs] = useState<SlogRecord[]>([]);

  useEffect(() => {
    void api
      .listLogs(userId)
      .then(setLogs)
      .catch(() => setLogs([]));
  }, [userId]);

  return (
    <section className="panel">
      <h2>Logs</h2>
      <ul className="list">
        {logs.map((log) => (
          <li key={log.id}>
            <span>{log.message}</span>
            <span className="muted">{log.created_at}</span>
          </li>
        ))}
        {logs.length === 0 && <li className="is-empty">No activity recorded.</li>}
      </ul>
    </section>
  );
}
