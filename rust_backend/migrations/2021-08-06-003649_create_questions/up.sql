-- Your SQL goes here
CREATE TABLE questions (
  question_id Serial PRIMARY KEY,
  question_type TEXT NOT NULL,
  question_text TEXT NOT NULL,
  correct_answer TEXT NOT NULL,
  wrong_answer_1 TEXT NOT NULL,
  wrong_answer_2 TEXT NOT NULL,
  wrong_answer_3 TEXT NOT NULL
);

INSERT INTO questions 
  (question_type, question_text, correct_answer, wrong_answer_1, wrong_answer_2, wrong_answer_3)
VALUES 
  ('multichoice', 'A butterflys wings are covered in thousands of tiny _____ ?','Scales','Feathers','Spots', 'Stripes'),
  ('multichoice', 'What is the name of the body part that a butterfly drinks with?', 'Proboscis', 'Mouth', 'Antenna', 'Straw'),
  ('multichoice', 'Whats the name of the 3rd stage of butterflys life cycle?', 'Pupa', 'Cacoon', 'Egg', 'Caterpillar'),
  ('multichoice', 'How many legs does a butterfly have?', '6', '4', '2', '101'),
  ('multichoice', 'What estimated percentage of all species exist in the Tropical Rainforest?', '80%', '60%', '50%', '10%'),
  ('multichoice', 'What do butterflies eat?', 'Nectar', 'Pollen', 'Leaves', 'Insects'),
  ('true/false', 'Is it hotter at the top of the Tropical Forest and cooler at the bottom?', 'True', 'False', '', ''),
  ('true/false', 'Butterflies use their feet to taste?', 'True', 'False', '', ''),
  ('multichoice', 'What do Tarantulas use to fend of predators?', 'Their barbed hairs', 'Their fangs', 'Their venom', 'Their size');
