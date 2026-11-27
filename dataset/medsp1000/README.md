---
license: mit
language:
- en
task_categories:
- text-generation
- question-answering
tags:
- medical
- clinical
- healthcare
- standardized-patients
- agent
- interactive-evaluation
- benchmark
- acgme
pretty_name: MedSP1000
size_categories:
- 1K<n<10K
---

# MedSP1000

### Evaluating Large Language Models in Dynamic Clinical Decision-Making with Standardized Patient Cases

[![Paper](https://img.shields.io/badge/paper-arXiv-b31b1b.svg)](https://arxiv.org/abs/2606.05112)
[![Code](https://img.shields.io/badge/GitHub-code-181717.svg)](https://github.com/MAGIC-AI4Med/MedSP1000)

Paper: [arxiv.org/abs/2606.05112](https://arxiv.org/abs/2606.05112) &nbsp;|&nbsp; Code: [github.com/MAGIC-AI4Med/MedSP1000](https://github.com/MAGIC-AI4Med/MedSP1000)

---

## Dataset Summary

**MedSP1000** is a standardized-patient (SP)–derived **interactive** benchmark for evaluating large
language models as clinical agents. Unlike static, single-turn medical QA, each item is an executable
**multi-turn encounter**: a clinician agent interacts in a closed loop with a patient agent and an
environment controller across successive patient states, and its behaviour is scored throughout the
encounter against an expert-defined, human-validated rubric.

The benchmark is built directly on peer-reviewed [MedEdPORTAL](https://www.mededportal.org/) SP
teaching materials, converting heterogeneous source articles into role-specific scenario packets via
an agentic data-processing pipeline.

## Highlights

- 🏥 **SP-grounded** — derived from peer-reviewed [MedEdPORTAL](https://www.mededportal.org/) teaching materials.
- 🔁 **Interactive, multi-turn** — closed-loop encounters between a clinician agent, a patient agent, and an environment controller, with a standardized state-transition protocol.
- 📊 **Scale & breadth** — **1,638 interactive cases** across **17 clinical specialties**, scored with **24,602 rubric items**.
- 🧭 **ACGME-aligned scoring** — every action graded against a frozen rubric over the **6 ACGME core competencies** (PC, MK, SBP, ICS, PBLI, PROF).
- 👩‍⚕️ **Human-validated** — cases and trajectories checked by clinicians with independent double-scoring.

## Dataset Structure

Each scenario packet holds the materials for the four role agents, laid out as a directory tree:

```
<case_id>/
  <scenarioN>/
    environment_controller/   # scene / environment-control materials
    evaluator/                # ACGME rubric, faculty & student evaluation forms
    examinee/                 # clinician (candidate)-facing materials
    sp_actor/                 # standardized-patient acting script
```

- `<case_id>` maps back to its source MedEdPORTAL article (e.g. `mep_2374-8265.10849-s001`).
- A case may contain several scenarios; `scenario_index.json` lists every scenario path in this repo.

## Usage

This dataset is a **structured file tree** (not a `datasets`-loadable tabular format). Download the
files, then run them with the simulation & ACGME-scoring harness in our GitHub repository:
👉 **[MAGIC-AI4Med/MedSP1000](https://github.com/MAGIC-AI4Med/MedSP1000)**

```python
from huggingface_hub import snapshot_download

# download all case/scenario files
path = snapshot_download(repo_id="byrLLCC/MedSP1000", repo_type="dataset")
# `path` now contains <case_id>/<scenarioN>/<role>/*.md
```

Then follow the instructions in the [GitHub repo](https://github.com/MAGIC-AI4Med/MedSP1000) to run the
interactive multi-turn encounters and rubric-based evaluation.

## Source & Provenance

Source cases are drawn from [MedEdPORTAL](https://www.mededportal.org/). Each released case carries a
traceable mapping back to its original MedEdPORTAL article (standard DOI link of the form
`https://www.mededportal.org/doi/10.15766/mep_2374-8265.<id>`) for citation and verification.

## Scoring

Scoring follows the [ACGME Core Competencies](https://www.acgme.org/). Each clinician action is graded
against a frozen rubric over the six competencies — Patient Care (PC), Medical Knowledge (MK),
Systems-Based Practice (SBP), Interpersonal & Communication Skills (ICS), Practice-Based Learning &
Improvement (PBLI), and Professionalism (PROF). A run's score is the fraction of expert-defined rubric
items completed across the full trajectory.

## Citation

```bibtex
@misc{liang2026evaluatinglargelanguagemodels,
      title={Evaluating Large Language Models in Dynamic Clinical Decision-Making with Standardized Patient Cases}, 
      author={Cheng Liang and Pengcheng Qiu and Ya Zhang and Yanfeng Wang and Chaoyi Wu and Weidi Xie},
      year={2026},
      eprint={2606.05112},
      archivePrefix={arXiv},
      primaryClass={cs.CL},
      url={https://arxiv.org/abs/2606.05112}, 
}
```

## License

Released under the MIT License.

## Acknowledgements

Source cases are drawn from [MedEdPORTAL](https://www.mededportal.org/). Scoring follows the
[ACGME Core Competencies](https://www.acgme.org/).
