(define (domain tcps-application-pipeline)
  (:requirements :strips :negative-preconditions :probabilistic-effects)
  (:predicates
    (fit-ready)
    (submitted)
    (clarification)
    (interview)
    (incompatible)
    (offer)
    (closed))

  (:action manufacture-fit-package
    :parameters ()
    :precondition (not (fit-ready))
    :effect (fit-ready))

  (:action recruiter-submit
    :parameters ()
    :precondition (and (fit-ready) (not (submitted)) (not (closed)))
    :effect (probabilistic
      0.65 (submitted)
      0.20 (clarification)
      0.15 (closed)))

  (:action answer-clarification
    :parameters ()
    :precondition (clarification)
    :effect (and (not (clarification)) (submitted)))

  (:action client-review
    :parameters ()
    :precondition (and (submitted) (not (interview)) (not (incompatible)))
    :effect (probabilistic
      0.55 (interview)
      0.15 (incompatible)))

  (:action production-assessment
    :parameters ()
    :precondition (and (interview) (not (offer)))
    :effect (probabilistic 0.60 (offer))))
