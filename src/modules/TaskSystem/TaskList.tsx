import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";
import { TaskComponent } from "./TaskComponent"

export const TaskList = () => {

  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    // Fetch tasks from the Rust backend
    const fetchTasks = async () => {
      try {
        const tasksData = await invoke("get_tasks");
        setTasks(tasksData as Task[]); // Cast to the appropriate type
      } catch (error) {
        console.error("Error fetching tasks:", error);
      }
    };

    fetchTasks();
  }, []);

  return (
    <div>
      {
        tasks.length > 0 ?
          tasks.map((task, key) => <TaskComponent task={task} key={key} />) :
          <div> No todo found...</div>
      }
    </div>
  )
}
