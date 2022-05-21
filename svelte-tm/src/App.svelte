<script>
  import FeedbackForm from './components/FeedbackForm.svelte'
  import FeedbackList from './components/FeedbackList.svelte'
  import FeedbackStas from './components/FeedbackStats.svelte'

  const addFeedback = (e) => {
    const newFeedback = e.detail
    feedback = [newFeedback, ...feedback]
  }
</script>

<main class="container">
  <FeedbackForm />
  <FeedbackStas />
  <FeedbackList />
</main>
