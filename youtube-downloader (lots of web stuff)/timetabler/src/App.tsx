

function App() {
  const getTheThing = async () => {
    const student_group = "woa"
    const which_week = 26
    const res = await fetch(`/api/timetable?student_group=${student_group}&which_week=${which_week}`)
    const text = await res.text()

    console.log(text)
  }

  return (
    <>
      <h1 className='bg-red-400'>hi</h1>

      <button onClick={getTheThing}>Click this</button>
    </>
  )
}

export default App
