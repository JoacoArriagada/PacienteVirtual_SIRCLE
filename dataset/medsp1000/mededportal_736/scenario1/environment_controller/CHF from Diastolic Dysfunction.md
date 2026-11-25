# 1. **Title**: A human-patient simulation of a patient with undiagnosed congestive heart failure from diastolic dysfunction who presents preoperatively for an elective trans-urethral resection of the prostate (TURP). 

# 3. **Overview of the scenario**:

## 3.2. <u>Simulation</u>:

### 3.2.1. *The simulated patient* (written for a Laerdal® SimMan™) is in a preoperative holding area.  Participants are to perform a history and physical exam, recognize the patient’s CHF, cancel the surgery, consult with the urologist and patient’s primary care physician, and notify the OR staff that the patient will not be having surgery. We anticipate that a number of participants will begin treatment of the patient’s hypertension and CHF.

### 3.2.3. Consenting the patient to anesthesia is a “fail” in the scenario. The simulation faculty has the options of stopping the scenario and doing a complete debriefing then, or, as we choose, may review the history and physical with the participant as the “attending who has just shown up,” and help the participant recognize the situation.  Once that recognition has occurred we restart the scenario and let the participant continue.

## 3.3. Environment

### 3.3.1. *Lab Set Up* –A pre-operative holding area

### 3.3.2. *Manikin Set Up* – This scenario was written for a SimManTM simulator.   We do not use moulage or make up to make the mannequin appear cyanotic, but that could be tried. The mannequin is sitting up right and unmonitored.  It has a respiratory rate of 24 breaths/minute, crackles for breath sounds, a BP of 180/110, an SpO2 of 90%, and a heart rate of 110 with normal sinus rhythm and left ventricular hypertrophy QRS morphology.  We would program an S4 heart sound, if that were available.

### 3.3.3. *Props* – A peripheral IV, simple face mask, and oxygen source are needed.  Drugs programmed for use in the simulation include metoprolol, esmolol, labetalol, nitroprusside, hydralazine, fenoldopam, verapamil, sublingual nifedipine, furosemide, enalapril, and phenylephrine.  A 12-lead ECG (displayed below) shows sinus tachycardia, left axis deviation, and LVH with strain.  A chest X-ray (below) shows mild to moderate CHF. Electrolyte and blood count lab values are available, as described below.  Video recording capability should be available to record the learner’s experience for subsequent review and evaluation.

## 3.4. Actors

### 3.4.1. *Roles* –Holding-area nurse, patient voice, urologist, OR staff, attending anesthesiologist, and primary care physician.

### 3.4.3. *Action Role* 

#### 3.4.3.1. Nurse: orients and proctors the participants.  Provides requested medications, equipment and laboratory studies.

#### 3.4.3.2. Patient: answers questions as per scripted H & P.

#### 3.4.3.3. Urologist: listens to participant’s presentation and asks about treatment options and rescheduling procedure.

#### 3.4.3.4. Primary Care Physician: listens to presentation and collaborates on patient care.  Can ask questions related to further preoperative workup and optimization before rescheduling. 

#### 3.4.3.5. OR staff: Acknowledge the participant’s notification.

# 5. **Events and Programmed Responses:** Three handlers (ABC Events, Medication Events, and Miscellaneous Events) run this program. Once an event is clicked the time it occurred is noted in the simulation’s performance log.  

The programmed responses triggered by an event are displayed as screen captures from the program.

**ABC Handler**

	**Event:** <u>Oxygen via simple face mask</u>

![](images/97a3ac5bce996cee0a055023b27b70bb64ae74f600ee2d1937151ff1c5f7b683.jpg)

**Trend:** 

**Rationale:** We’ve programmed this trend assuming that the patient has a mild amount of V/Q mis-matching, therefore this amount of supplementary oxygen will improve oxygen saturation as programmed.  Bag mask ventilation is unnecessary.

**Medication Handler**

**Event:** Hydralazine, 10 mg

![](images/ecf3a3b8b064580214b29369a30a248880a387792e40ca7ce8a8d04898ced69a.jpg)

**Trend:** 

**Event:** Nitroprusside infusion, 0.3 mcg-0.5 mcg/kg/min

![](images/43a71f24d7645e73a61aea3cfb172d84dc8638ab466357c30076e9cfff90ad6a.jpg)

**Trend:** 

**Event:** Furosemide, 20-40 mg

![](images/f52fc924ab42bf28a0fa982c36926d6bda84a77a80a5c18ecb49c1b4059e708f.jpg)

**Trend:** 

**Event:** Metoprolol, 1-2 mg, Metoprolol 2-5 mg, Metoprolol 5 mg and above

![](images/a375b873923eb1d400de45037b863f3962823f162dc15513af06a175ad582725.jpg)

**Trends:** 

![](images/c2f026cb6399dd3da0537e873769f8f40d61ed7686de201a7628579fca2f54b3.jpg)

![](images/42fa86528ab6add91ff98258304ba5e7de76eed412ae660b1a37ea4c51e26986.jpg)

**Event:** Esmolol, 40 mg bolus

**Trend:**

![](images/83ae0a8ea79b60db5094bd806062cad4f35ab738057c0c03fb2d905364f426ac.jpg)

**Event:** Labetalol, 10-20 mg, Labetalol, 20-40 mg, Labetalol, 40 mg and above

![](images/724902950eafb40219ad6e822e08061f993eecdf58fbdca6b4035fbdab2bd4be.jpg)

**Trend:** 

![](images/a6af138fd85a65dc87f6adfc0664b14e3f6302ee458dfe82a2af462acc623774.jpg)

![](images/be53ac62d79814f00e7787359ae3ac3e648da3bfab6d165538d8721dc74c1e7a.jpg)

**Event:** Verapamil, 10 mg

![](images/309307ccf08cf1ab5a4c70b44c96e00dec9d8c06ee82d46cf365da2b806e43df.jpg)

**Trend:** 

**Event:** Nifedipine (sublingual)

![](images/5fa8d0e0dac1c683584987ce6ac57c7b5986746995ed8b814732d940a604439b.jpg)

**Trend:** 

**Event:** Fenoldopam, 0.1-0.3 mcg/kg/min

![](images/d98c4b6992f06d4dc4f4923dbdfff8fc3e8b955e526f5f1b2db2b1f92f061f60.jpg)

**Trend:**	

**Event:** Enalapril, 2 mg

![](images/2394f821e558f6bb01f3ae1c493192de88d97876cf7d563c3682c532cc4db3a5.jpg)

**Trend:** 

**Miscellaneous Handler**

**Event:** <u>Greeting</u>

**Action:** Marks time T=0 for the scenario.

**Event:** <u>Consent to Anesthesia</u>

**Action:** Fails the scenario

**Event:** <u>Monitors (ECG, non-invasive BP cuff, and pulse oximeter) placed</u>

**Action:** In the SimMan application, clicking “NIBP” or “ECG,” or placing a pulse oximeter displays those vital signs on the simulator’s monitor, and enters the time that action occurred into the performance log.

**Event:** <u>History performed</u>

**Actions:** Denotes the time participants begin taking the history in the simulation log. Scripted answers to history questions are attached.

**Event:** <u>Physical examination</u> 

**Actions:** Denotes the time participants begin performing a physical exam in the simulation log. Starting vital signs and physical findings are previously described.  Note that SimManTM’s breath sounds are difficult to interpret.  After auscultating the lungs the simulation’s facilitator should ask what is heard and correct any misinterpretations.

If queried, the patient does not have prominent peripheral edema, jugular venous distension, or hepatojugular reflux.

**Event:** <u>Order an ECG</u>:

**Actions:** The following ECG6 becomes available a few minutes after being ordered.

![](images/4b970282526d9bdbf5bd48b45e6e28d45adb05a779b0afdf8ec892b844c90a40.jpg)

**Event:** <u>Order a chest x-ray (CXR)</u>:

**Actions:** The proceeding CXR becomes available a few minutes after being ordered:

![](images/007a70dd719d4951880f05e7ff61e250b313fff5368086abb42352a6a41423cb.jpg)

**Event:** <u>Order electrolytes or arterial blood gas (ABG)</u>:

**Actions:** Values become available a few minutes after being ordered.

Electrolytes: Na 141, K 3.8, Cl 120, CO2 24, BUN 26, Cr 1.3, glucose 115, ABG: pH7.39/PaCO237/PaO262 before oxygen therapy or 7.39/35/70 after.

**Event:** <u>Cancel the case</u>

**Action:** This event is noted in the log.

**Event:** <u>Call urologist</u>

**Action:** The event is noted in the log.  An actor (the simulation facilitator) takes the call and is genial and agreeable.

**Event:** <u>Call OR staff</u>

**Action:** The event is noted in the log.  The OR staff thanks the participant for the communication.

**Event:** <u>Call Patient’s PCP</u>

**Action:** The event is noted in the log.  Our facilitator plays the role of the PCP, and discusses follow up care with the participant.  Typically, we ask the participant what studies they suggest and what criterion they would use to determine when the patient is ready for surgery.

**Event:** <u>Discuss findings and plan with the patient</u>

**Action:** The event is noted in the log.  The “patient” acknowledges the participants’ care and concern.

**Attachments**

    1. History
        1. HPI
            1. Does patient complain of dyspnea?  Yes
            2. Is there dyspnea only on exertion, or at rest too?  Both
            3. What is the time course of the dyspnea?  3 months.  I have been short of breath at rest only for a few days.
            4. What is the patient’s functional capacity?  I work on a farm, and am usually able to carry a bale of hay without stopping.  However, I cannot do what I once did because of my breathing, and lately I haven’t been able to work.
        2. PMH
            1. Is there any past medical history sir?  Hypertension
            2. Anything else, such as heart attack or chest pain or diabetes?  No
            3. Emphysema or asthma or COPD?  No
            4. Ever a smoker?  As a teen, none for 60 years.
            5. Any other lung diseases (IPF, pneumonia, chemical or dust exposure, etc.)?  No.
            6. Any kidney or liver problems?  No.
        3. PSH
            1. Any surgeries in the past?  Inguinal hernia repair 20 years ago
            2. Any anesthetic complications?  No.
        4. Medications
            1. What medications do you take?  Lisinopril 25 mg PO daily
            2. When did you start?  5 years ago.
            3. And you’ve never had a cough?  No.
            4. Has your tongue every swollen on you or have you gotten all puffy?  No.
        5. Allergies
            1. Any allergies?  None
        6. FH and SH
            1. Do you smoke?  No
            2. Have you ever smoked?  No
            3. Do you drink or have you ever?  Yes.
            4. What’s occasional?  1 beer a day.
            5. Any illicit drug use?  No.
            6. Employment?  Farmer
            7. Living conditions?  Three story house
            8. Relationship status?  Married with 3 grown kids.
            9. Any family history for MI, diabetes, CHF, CP, ACS or CAD?  No.
            10. Any family history of cancer?  No.
            11. Any family history of asthma?  No.
        7. ROS
            1. Viral infection or other illness, syncopcal episodes, coughing, mucus, CP, dyspnea on exertion, PND, nocturia
                1. The patient complains of PND and sleeps on 3 pillows
                2. The patient complains of nocturia, usually getting up three times per sleeping.
                3. Dyspnea on exertion as above.
            2. No coughing, mucus production, sickness or sick contacts, CP, orthopnea, MI, or history of CHF
