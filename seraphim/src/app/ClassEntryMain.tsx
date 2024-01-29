'use client'
import React, { Component } from 'react';
import StudentSearchModal from './StudentSearchModal.tsx';
import ClassTable from './ClassTable.tsx';

class ClassEntryMain extends Component {
  constructor(props) {
    super(props);
    this.nextClass = this.nextClass.bind(this);
    this.backClass = this.backClass.bind(this);
    this.updateTable = this.updateTable.bind(this);
    this.state = {class_id: this.props.id, class_entry_data: [], class_type_data: [], instructor_data: [], student_table: [] };
  }

  async componentDidMount() {
    const response = await fetch('/class/' + this.state.class_id, { method: 'GET' })
    let json = await response.json();
    const class_response = await fetch('/class-type/' + json.class_id, { method: 'GET' });
    let class_json = await class_response.json();
    const instructor_response = await fetch('/instructor/' + json.instructor_id, { method: 'GET' });
    let instructor_json = await instructor_response.json();
    this.setState({ class_entry_data: json, class_type_data: class_json, instructor_data: instructor_json });
  }

  async updateTable() {
    const response = await fetch('/class/' + this.state.class_id + '/students', { method: 'GET' });
    let json = await response.json();
    for (let s in json) {
      const student_response = await fetch('/student/' + json[s].student_id, { method: 'GET' });
      let student_json = await student_response.json();
      json[s]['student_data'] = student_json;
      const company_response = await fetch('/company/' + json[s].student_data.company_id, { method: 'GET' });
      let company_json = await company_response.json();
      json[s]['company_data'] = company_json;
    }

    this.setState({ student_table: json });
  }

  async nextClass(next_id) {
    const response = await fetch('/class/' + next_id, { method: 'GET' });
    let json = await response.json();
    const class_response = await fetch('/class-type/' + json.class_id, { method: 'GET' });
    let class_json = await class_response.json();
    const instructor_response = await fetch('/instructor/' + json.instructor_id, { method: 'GET' });
    const instructor_json = await instructor_response.json();
    this.setState({ class_id: next_id, class_entry_data: json, class_type_data: class_json, instructor_data: instructor_json });
    
    return this.state.class_id;
  }

  async backClass(back_id) {
    const response = await fetch('/class/' + back_id, { method: 'GET' });
    let json = await response.json();
    const class_response = await fetch('/class-type/' + json.class_id, { method: 'GET' });
    let class_json = await class_response.json();
    const instructor_response = await fetch('/instructor/' + json.instructor_id, { method: 'GET' });
    let instructor_json = await instructor_response.json();
    this.setState({ class_id: back_id, class_entry_data: json, class_type_date: class_json, instructor_data: instructor_json });

    return this.state.class_id;
  }  
  
  render() {
    return(
      <>
        <div class="block max-w-sm rounded-lg bg-white w-full lg:max-w-full lg:flex shadow-lg">
          <div class="grid grid-cols-1 gap-4 p-8 content-center justify-start">
	    <div class="md:flex md:items-center mb-2">
  	      <div class="md:w-3/12">
      	        <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
	          Class Type
                </label>
              </div>
              <div class="md:w-2/3">
                <input class="md:text-center bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="inline-full-name" type="text" value="Placeholder"/>
              </div>
  	      <div class="md:w-4/12">
      	        <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
	          Class Date
                </label>
              </div>
              <div class="md:w-4/12">
                <input class="md:text-center bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="inline-full-name" type="text" value="Placeholder"/>
              </div>
  	      <div class="md:w-4/12">
      	        <label class="block text-gray-500 font-bold md:text-center mb-1 md:mb-0 pr-4" for="inline-full-name">
	          <text style={{ marginLeft: '1rem' }}>to</text>
                </label>
              </div>
              <div class="md:w-4/12">
                <input class="md:text-center bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="inline-full-name" type="text" value="Placeholder"/>
              </div>
            </div>
	    <div class="md:flex md:items-center mb-2">
  	      <div class="md:w-3/12">
      	        <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
	          Instructor
                </label>
              </div>
              <div class="md:w-2/3">
                <input class="md:text-center bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="inline-full-name" type="text" value="Placeholder"/>
              </div>
  	      <div class="md:w-4/12">
      	        <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
	          Recert Date
                </label>
              </div>
              <div class="md:w-4/12">
                <input class="md:text-center bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="inline-full-name" type="text" value="Placeholder"/>
              </div>
  	      <div class="md:w-4/12">
      	        <label class="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4" for="inline-full-name">
	          Recert Date (DC)
                </label>
              </div>
              <div class="md:w-4/12">
                <input class="md:text-center bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500" id="inline-full-name" type="text" value="Placeholder"/>
              </div>
            </div>
          </div>
	</div>
        <div class="block max-w-sm rounded-lg bg-white w-full lg:max-w-full lg:flex shadow-lg">
          <div class="grid grid-cols-1 gap-4 p-8 content-center justify-start">
	    <div class="md:flex md:items-center mb-2">
	      <StudentSearchModal />
            </div>
          </div>
        </div>
	<ClassTable />
      </>
    );
  }
}

export default ClassEntryMain;
