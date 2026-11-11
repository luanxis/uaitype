import { FAQS } from '../data'

export const Faq = () => (
  <section id="faq">
    <p class="sec-label">faq</p>
    <h2>Perguntas frequentes</h2>
    <div class="faq-list">
      {FAQS.map((faq) => (
        <details class="faq-item" name="faq">
          <summary>
            <span>{faq.question}</span>
            <span class="faq-marker" aria-hidden="true">+</span>
          </summary>
          <div class="faq-answer">
            <p>{faq.answer}</p>
          </div>
        </details>
      ))}
    </div>
  </section>
)
