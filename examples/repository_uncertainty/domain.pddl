(define (domain repository-uncertainty)
  (:requirements :strips :negative-preconditions :probabilistic-effects)
  (:predicates
    (validated)
    (defect)
    (published))

  (:action inspect-candidate
    :parameters ()
    :precondition (and (not (validated)) (not (defect)))
    :effect (probabilistic
      0.75 (validated)
      0.25 (defect)))

  (:action repair-defect
    :parameters ()
    :precondition (defect)
    :effect (and (not (defect)) (validated)))

  (:action publish
    :parameters ()
    :precondition (validated)
    :effect (published)))
