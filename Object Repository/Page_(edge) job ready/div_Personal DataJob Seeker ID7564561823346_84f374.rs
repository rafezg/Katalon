<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Personal DataJob Seeker ID7564561823346_84f374</name>
   <tag></tag>
   <elementGuidId>5b26ee46-039c-4ebb-856c-ca83645f8208</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.row.details.margin-top-lg</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//section[@id='main-content']/section[2]/div[2]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>52ea0d1f-155f-4ac6-bd13-e3cffbe04fba</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>row details margin-top-lg</value>
      <webElementGuid>d5df0699-0d1d-4f3b-88cf-d6f9143f0653</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>




Personal Data












Job Seeker ID
75645618233468457
Preferred Name

Date of Birth
 
Interpreter Required
None
Primary Contact

Katalon Test

?


Direct Registration
No
Bank Account Name

BSB

Bank Account Number





Address




Email Address


Email

Phone

Mobile




Preferred Contact Method









Comments














Key Information





Status
Pending
Phase

Funding Level

Program

Benchmark Hours

Registered By

Managed By

Customer Coordinator

ESS Next Diary Appointment Date
No appointment set
Site
ACE (ACE)
Specialist Type














Alerts















ESS Data











Contract ID

CRN

Benchmark Hours

Site
ACE (ACE)
Status
Pending
Phase

Funding Level

Program

Disability Type

Allowance Type

Allowance Rate

Participation Requirement
No
Time with Provider
0 days
Time with Site
0 days
Job Plan Status

Job Plan Status Date

SWS Wage Agreement Signed Date

Period of Service

Principle Carer Parent

Anchor Date

Actual Program Review Completion Date

Employment Start Date

DES Transition Transfer

Wage Subsidy Eligible
No
Ongoing Support Assessment Date

Flexible OS Instances

Exit Reason

Legal Requirement

Wage Subsidy Type





Suspension Start Date

Expected Suspension End Date

Actual Suspension End Date

Suspension Reason

JSCI Status

JSCI Last Updated Date

JSCI Last Updated By

Education Level

Temporary Work Capacity 1

Temporary Work Capacity 2

CALD

Mature Age

Vulnerable Youth

Volunteer
No
Indigenous
No
Refugee Humanitarian Visa

Eligible School Leaver

Exit Date

Program Review Due Date

Referral Status
Referred (REF)
Moderate Intellectual Disability
No
Ongoing Support Next Payment Date

Restart Eligibility
No
Ongoing Support Assessment Due Date

ESAT Assessment Date

Period of Service Start Date (beta)













Info


































Provider Tags


0







Tag Id
Description
Category
Updated By
Start Date
End Date













Activities Stream

Attitudinal Surveys


Reminders
0



Vacancies

0


Appointments
0


SMS
0


Letters
0


Emails
0


Indigenous Mentoring
0


Professional Services
0


Reverse Marketing
0


Job Match


Notes
0


Expenses
0


Forms
0


Claims


Resumes
0


Job Seeker Portal Info


Max Connects


Attachments
0



Contacts


Customer Support Plan
0


Wage Subsidies
0


Queries
0









Reminders
 



Current


Completed



















Notes
 



Print All File Notes
Print Selected File Notes



Formatted Comments




No items to display here!


Occurred At


User
Type
Name

Comments

Time Spent
Travel Time
Tags
Attachment
Reminder
Actions


  $('.note_select_for_printing').each(function() {
    if ( (NEPTUNE.print_selected_note_ids.indexOf(this.value) !== -1) ) {
      this.checked = true;
    }
  });
  
  $(&quot;.click-popover&quot;).on('click', function(e) {
    e.preventDefault();
  });



No Notes



  function setCookie(c_name,value,expiredays) {
    var exdate=new Date()
    exdate.setDate(exdate.getDate()+expiredays)
    document.cookie=c_name+ &quot;=&quot; +escape(value)+((expiredays==null) ? &quot;&quot; : &quot;;expires=&quot;+exdate)
  }
  
  function getCookie(c_name) {
    if (document.cookie.length>0) {
      c_start=document.cookie.indexOf(c_name + &quot;=&quot;)
      if (c_start!=-1) {
          c_start=c_start + c_name.length+1
          c_end=document.cookie.indexOf(&quot;;&quot;,c_start)
          if (c_end==-1) c_end=document.cookie.length
              return unescape(document.cookie.substring(c_start,c_end))
      }
    }
    return null
  }
  
  $('#jobseekerNotesToggleCheckbox').prop('checked', getCookie('noteCommentsToggleCheckbox'));
  
  $('#jobseekerNotesToggleCheckbox').on(&quot;change&quot;, function(e){
    setCookie(&quot;noteCommentsToggleCheckbox&quot;, e.target.checked, 1);
  });
  
  $(document).on('osom-table:loaded', function() {
    $(&quot;#jobseekerNotesToggleCheckbox&quot;).trigger(&quot;click&quot;);
    $(&quot;#jobseekerNotesToggleCheckbox&quot;).trigger(&quot;click&quot;);
  });
  
  if ('ontouchstart' in document.documentElement) {
    $('.content-wrapper').css(&quot;cursor&quot;, &quot;pointer&quot;);
  }









Professional Service Activities
 Health Referral
 Internal Programs



Health Referrals


Program Referrals


Activities


















Portal Info


Privacy Documents











Portal Info


Privacy Documents












Customer Support Plan Status: To Do








Overarching Goal













Goals
 

Customer Support Plan
 







Open


Closed










Customer Support Plan Reminders
 




Email Selected
Email Customer Support Plan





All


Current


Completed






















</value>
      <webElementGuid>6e7f8e7d-a744-44f7-beb4-fa732c289c2a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;main-content&quot;)/section[@class=&quot;content-wrapper&quot;]/div[@class=&quot;row details margin-top-lg&quot;]</value>
      <webElementGuid>50ca0a8f-5317-4ef3-a109-dd53aad9fa09</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//section[@id='main-content']/section[2]/div[2]</value>
      <webElementGuid>0fa41f06-39ef-4542-bb49-3dadcce185a7</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section[2]/div[2]</value>
      <webElementGuid>78ba6a2b-4823-4c55-98d4-4ac9e1332da0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;




Personal Data












Job Seeker ID
75645618233468457
Preferred Name

Date of Birth
 
Interpreter Required
None
Primary Contact

Katalon Test

?


Direct Registration
No
Bank Account Name

BSB

Bank Account Number





Address




Email Address


Email

Phone

Mobile




Preferred Contact Method









Comments














Key Information





Status
Pending
Phase

Funding Level

Program

Benchmark Hours

Registered By

Managed By

Customer Coordinator

ESS Next Diary Appointment Date
No appointment set
Site
ACE (ACE)
Specialist Type














Alerts















ESS Data











Contract ID

CRN

Benchmark Hours

Site
ACE (ACE)
Status
Pending
Phase

Funding Level

Program

Disability Type

Allowance Type

Allowance Rate

Participation Requirement
No
Time with Provider
0 days
Time with Site
0 days
Job Plan Status

Job Plan Status Date

SWS Wage Agreement Signed Date

Period of Service

Principle Carer Parent

Anchor Date

Actual Program Review Completion Date

Employment Start Date

DES Transition Transfer

Wage Subsidy Eligible
No
Ongoing Support Assessment Date

Flexible OS Instances

Exit Reason

Legal Requirement

Wage Subsidy Type





Suspension Start Date

Expected Suspension End Date

Actual Suspension End Date

Suspension Reason

JSCI Status

JSCI Last Updated Date

JSCI Last Updated By

Education Level

Temporary Work Capacity 1

Temporary Work Capacity 2

CALD

Mature Age

Vulnerable Youth

Volunteer
No
Indigenous
No
Refugee Humanitarian Visa

Eligible School Leaver

Exit Date

Program Review Due Date

Referral Status
Referred (REF)
Moderate Intellectual Disability
No
Ongoing Support Next Payment Date

Restart Eligibility
No
Ongoing Support Assessment Due Date

ESAT Assessment Date

Period of Service Start Date (beta)













Info


































Provider Tags


0







Tag Id
Description
Category
Updated By
Start Date
End Date













Activities Stream

Attitudinal Surveys


Reminders
0



Vacancies

0


Appointments
0


SMS
0


Letters
0


Emails
0


Indigenous Mentoring
0


Professional Services
0


Reverse Marketing
0


Job Match


Notes
0


Expenses
0


Forms
0


Claims


Resumes
0


Job Seeker Portal Info


Max Connects


Attachments
0



Contacts


Customer Support Plan
0


Wage Subsidies
0


Queries
0









Reminders
 



Current


Completed



















Notes
 



Print All File Notes
Print Selected File Notes



Formatted Comments




No items to display here!


Occurred At


User
Type
Name

Comments

Time Spent
Travel Time
Tags
Attachment
Reminder
Actions


  $(&quot; , &quot;'&quot; , &quot;.note_select_for_printing&quot; , &quot;'&quot; , &quot;).each(function() {
    if ( (NEPTUNE.print_selected_note_ids.indexOf(this.value) !== -1) ) {
      this.checked = true;
    }
  });
  
  $(&quot;.click-popover&quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    e.preventDefault();
  });



No Notes



  function setCookie(c_name,value,expiredays) {
    var exdate=new Date()
    exdate.setDate(exdate.getDate()+expiredays)
    document.cookie=c_name+ &quot;=&quot; +escape(value)+((expiredays==null) ? &quot;&quot; : &quot;;expires=&quot;+exdate)
  }
  
  function getCookie(c_name) {
    if (document.cookie.length>0) {
      c_start=document.cookie.indexOf(c_name + &quot;=&quot;)
      if (c_start!=-1) {
          c_start=c_start + c_name.length+1
          c_end=document.cookie.indexOf(&quot;;&quot;,c_start)
          if (c_end==-1) c_end=document.cookie.length
              return unescape(document.cookie.substring(c_start,c_end))
      }
    }
    return null
  }
  
  $(&quot; , &quot;'&quot; , &quot;#jobseekerNotesToggleCheckbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, getCookie(&quot; , &quot;'&quot; , &quot;noteCommentsToggleCheckbox&quot; , &quot;'&quot; , &quot;));
  
  $(&quot; , &quot;'&quot; , &quot;#jobseekerNotesToggleCheckbox&quot; , &quot;'&quot; , &quot;).on(&quot;change&quot;, function(e){
    setCookie(&quot;noteCommentsToggleCheckbox&quot;, e.target.checked, 1);
  });
  
  $(document).on(&quot; , &quot;'&quot; , &quot;osom-table:loaded&quot; , &quot;'&quot; , &quot;, function() {
    $(&quot;#jobseekerNotesToggleCheckbox&quot;).trigger(&quot;click&quot;);
    $(&quot;#jobseekerNotesToggleCheckbox&quot;).trigger(&quot;click&quot;);
  });
  
  if (&quot; , &quot;'&quot; , &quot;ontouchstart&quot; , &quot;'&quot; , &quot; in document.documentElement) {
    $(&quot; , &quot;'&quot; , &quot;.content-wrapper&quot; , &quot;'&quot; , &quot;).css(&quot;cursor&quot;, &quot;pointer&quot;);
  }









Professional Service Activities
 Health Referral
 Internal Programs



Health Referrals


Program Referrals


Activities


















Portal Info


Privacy Documents











Portal Info


Privacy Documents












Customer Support Plan Status: To Do








Overarching Goal













Goals
 

Customer Support Plan
 







Open


Closed










Customer Support Plan Reminders
 




Email Selected
Email Customer Support Plan





All


Current


Completed






















&quot;) or . = concat(&quot;




Personal Data












Job Seeker ID
75645618233468457
Preferred Name

Date of Birth
 
Interpreter Required
None
Primary Contact

Katalon Test

?


Direct Registration
No
Bank Account Name

BSB

Bank Account Number





Address




Email Address


Email

Phone

Mobile




Preferred Contact Method









Comments














Key Information





Status
Pending
Phase

Funding Level

Program

Benchmark Hours

Registered By

Managed By

Customer Coordinator

ESS Next Diary Appointment Date
No appointment set
Site
ACE (ACE)
Specialist Type














Alerts















ESS Data











Contract ID

CRN

Benchmark Hours

Site
ACE (ACE)
Status
Pending
Phase

Funding Level

Program

Disability Type

Allowance Type

Allowance Rate

Participation Requirement
No
Time with Provider
0 days
Time with Site
0 days
Job Plan Status

Job Plan Status Date

SWS Wage Agreement Signed Date

Period of Service

Principle Carer Parent

Anchor Date

Actual Program Review Completion Date

Employment Start Date

DES Transition Transfer

Wage Subsidy Eligible
No
Ongoing Support Assessment Date

Flexible OS Instances

Exit Reason

Legal Requirement

Wage Subsidy Type





Suspension Start Date

Expected Suspension End Date

Actual Suspension End Date

Suspension Reason

JSCI Status

JSCI Last Updated Date

JSCI Last Updated By

Education Level

Temporary Work Capacity 1

Temporary Work Capacity 2

CALD

Mature Age

Vulnerable Youth

Volunteer
No
Indigenous
No
Refugee Humanitarian Visa

Eligible School Leaver

Exit Date

Program Review Due Date

Referral Status
Referred (REF)
Moderate Intellectual Disability
No
Ongoing Support Next Payment Date

Restart Eligibility
No
Ongoing Support Assessment Due Date

ESAT Assessment Date

Period of Service Start Date (beta)













Info


































Provider Tags


0







Tag Id
Description
Category
Updated By
Start Date
End Date













Activities Stream

Attitudinal Surveys


Reminders
0



Vacancies

0


Appointments
0


SMS
0


Letters
0


Emails
0


Indigenous Mentoring
0


Professional Services
0


Reverse Marketing
0


Job Match


Notes
0


Expenses
0


Forms
0


Claims


Resumes
0


Job Seeker Portal Info


Max Connects


Attachments
0



Contacts


Customer Support Plan
0


Wage Subsidies
0


Queries
0









Reminders
 



Current


Completed



















Notes
 



Print All File Notes
Print Selected File Notes



Formatted Comments




No items to display here!


Occurred At


User
Type
Name

Comments

Time Spent
Travel Time
Tags
Attachment
Reminder
Actions


  $(&quot; , &quot;'&quot; , &quot;.note_select_for_printing&quot; , &quot;'&quot; , &quot;).each(function() {
    if ( (NEPTUNE.print_selected_note_ids.indexOf(this.value) !== -1) ) {
      this.checked = true;
    }
  });
  
  $(&quot;.click-popover&quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
    e.preventDefault();
  });



No Notes



  function setCookie(c_name,value,expiredays) {
    var exdate=new Date()
    exdate.setDate(exdate.getDate()+expiredays)
    document.cookie=c_name+ &quot;=&quot; +escape(value)+((expiredays==null) ? &quot;&quot; : &quot;;expires=&quot;+exdate)
  }
  
  function getCookie(c_name) {
    if (document.cookie.length>0) {
      c_start=document.cookie.indexOf(c_name + &quot;=&quot;)
      if (c_start!=-1) {
          c_start=c_start + c_name.length+1
          c_end=document.cookie.indexOf(&quot;;&quot;,c_start)
          if (c_end==-1) c_end=document.cookie.length
              return unescape(document.cookie.substring(c_start,c_end))
      }
    }
    return null
  }
  
  $(&quot; , &quot;'&quot; , &quot;#jobseekerNotesToggleCheckbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, getCookie(&quot; , &quot;'&quot; , &quot;noteCommentsToggleCheckbox&quot; , &quot;'&quot; , &quot;));
  
  $(&quot; , &quot;'&quot; , &quot;#jobseekerNotesToggleCheckbox&quot; , &quot;'&quot; , &quot;).on(&quot;change&quot;, function(e){
    setCookie(&quot;noteCommentsToggleCheckbox&quot;, e.target.checked, 1);
  });
  
  $(document).on(&quot; , &quot;'&quot; , &quot;osom-table:loaded&quot; , &quot;'&quot; , &quot;, function() {
    $(&quot;#jobseekerNotesToggleCheckbox&quot;).trigger(&quot;click&quot;);
    $(&quot;#jobseekerNotesToggleCheckbox&quot;).trigger(&quot;click&quot;);
  });
  
  if (&quot; , &quot;'&quot; , &quot;ontouchstart&quot; , &quot;'&quot; , &quot; in document.documentElement) {
    $(&quot; , &quot;'&quot; , &quot;.content-wrapper&quot; , &quot;'&quot; , &quot;).css(&quot;cursor&quot;, &quot;pointer&quot;);
  }









Professional Service Activities
 Health Referral
 Internal Programs



Health Referrals


Program Referrals


Activities


















Portal Info


Privacy Documents











Portal Info


Privacy Documents












Customer Support Plan Status: To Do








Overarching Goal













Goals
 

Customer Support Plan
 







Open


Closed










Customer Support Plan Reminders
 




Email Selected
Email Customer Support Plan





All


Current


Completed






















&quot;))]</value>
      <webElementGuid>efb6d030-b139-462d-a715-14f29050e230</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
