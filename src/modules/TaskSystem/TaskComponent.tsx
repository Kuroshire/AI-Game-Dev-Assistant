export const TaskComponent = ({ task } : { task: Task }) => {

  return (
    <div>
      <div>
        {task.title}
      </div>
      <div>
        Is Completed : {task.completed}
      </div>
    </div>
  )
}