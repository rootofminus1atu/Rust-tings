

function App() {
  const getTheThing = async () => {
    const test_param = "woa"
    const res = await fetch(`/api/add?search_term=${test_param}`)
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
