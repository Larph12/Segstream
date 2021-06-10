<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Dashboard  SegStream                  _0b2a3b</name>
   <tag></tag>
   <elementGuidId>c69f2e89-cd78-40dd-9fda-29837e5361ba</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    Dashboard | SegStream
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    

    
    
    

    
    
    
    
    
    
    


    var storage = window.localStorage;

    $(document).ready(function () {
        if($(&quot;#search&quot;).val() != &quot;&quot;){
            $(&quot;.search-form .form-group&quot;).addClass('hover');
        } else {
            $(&quot;.search-form .form-group&quot;).removeClass('hover');
        }

        $('[data-toggle=&quot;tooltip&quot;]').tooltip();

        $('#sidebarCollapse').on('click', function () {
            $('#sidebar').toggleClass('active');
            if($('#sidebar').hasClass('active')){
                $(&quot;#content&quot;).removeClass('content-active-block');
                $(&quot;#content&quot;).addClass('content-inactive-block');

            }else{
                $(&quot;#content&quot;).removeClass('content-inactive-block');
                $(&quot;#content&quot;).addClass('content-active-block');
            }
        });

        $('input[type=radio][name=view_mode]').change(function() {
            $(&quot;#view_mode_form&quot;).submit();
        });

        $(document).on('click', '#merge_submit_button', function(e){
            e.preventDefault();
            var properties_id = []
            $('.merged_properties_id').each(function(e){
                if($(this).is(':checked')){
                    var id = $(this).val();
                    properties_id.push(id)
                }
            });
            $('#properties_list_id').val(JSON.stringify(properties_id));
            $('#merge_properties_form').submit()
        });

        $(document).ready(function(){
            $('#id_select_property_merge').multiselect({
                buttonWidth: &quot;100%&quot;,
                nonSelectedText: 'Select',
                enableFiltering: true,
                enableCaseInsensitiveFiltering: true,
                maxHeight: 300,
            });

            $('[data-toggle=&quot;accordian&quot;]').each(function(){
                $(this).hide();
                var project_type = $(this).attr('data-shared');

                if(project_type == 'owner'){
                    $(this).find('.more-less').toggleClass('fa-folder-open fa-folder');
                }else{
                    $(this).find('.more-less').toggleClass('shared-folder-open shared-folder');
                }
                $(this).find('.more-less').closest('accordian').removeAttr('style');
            })
        })

        // $('#project_list_div [data-toggle=&quot;accordion&quot;]').on('click', function(e) {
        //     var target = $(this).attr('href');
        //     $(target).slideToggle(300);
        //     e.preventDefault();

        //     var data_type = $(this).attr('data-type');
        //     var data_type_id = $(this).attr('data-type-id');
        //     var project_type = $(this).attr('data-shared');

        //     if(project_type == 'owner'){
        //         $(this).find('.more-less').toggleClass('fa-folder-open fa-folder');
        //     }else{
        //         $(this).find('.more-less').toggleClass('shared-folder-open shared-folder');
        //     }
        //     $(this).find('.more-less').closest('accordian').removeAttr('style');
        // });

        $('#applyMerge [data-toggle=&quot;accordion&quot;]').on('click', function(e) {
            var target = $(this).attr('href');
            $(target).slideToggle(300);
            e.preventDefault();

            var data_type = $(this).attr('data-type');
            var data_type_id = $(this).attr('data-type-id');
            var project_type = $(this).attr('data-shared');

            if(project_type == 'owner'){
                $(this).find('.more-less').toggleClass('fa-folder-open fa-folder');
            }else{
                $(this).find('.more-less').toggleClass('shared-folder-open shared-folder');
            }
            $(this).find('.more-less').closest('accordian').removeAttr('style');
        });

        $('.delete-btn').each(function() {
            $(this).on('click', function(e) {
                e.preventDefault();
                e.stopPropagation();
                var id = $(this).attr('data-project-id');
                $(&quot;#id_delete_project_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_project(&quot; + id + &quot;)&quot;);
                $(&quot;#id_delete_project_modal&quot;).modal(&quot;show&quot;);
            })
        })
        $('.delete-entity').each(function() {
            $(this).on('click', function(e) {
                e.preventDefault();
                e.stopPropagation();
                var id = $(this).attr('data-entity-id');
                $(&quot;#id_delete_entity_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_entity(&quot; + id + &quot;)&quot;);
                $(&quot;#id_delete_entity_modal&quot;).modal(&quot;show&quot;);
            })
        })

        $(&quot;#id_select_project&quot;).change(function () {
            $(&quot;#append_property_name&quot;).html('')
            var value = this.value;
            $.ajax({
                url: &quot;/project/get-merged-enities/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'project_id': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;entity_name&quot; id=&quot;id_entity_name&quot;>\
                        &lt;option value=&quot;&quot;>Select&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].id+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].name  +&quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#append_entities_name&quot;).show();
                    $(&quot;#append_entities_name&quot;).html(select);
                }
            });
        });

        // $('#id_delete_property_modal').on('shown.bs.modal', function(e){
        //     $('#id_delete_confirm').focus();
        // });

        // $('#id_delete_entity_modal').on('shown.bs.modal', function(e){
        //     $('#id_delete_entity_confirm').focus();
        // });

        // $('#id_delete_project_modal').on('shown.bs.modal', function(e){
        //     $('#id_delete_project_confirm').focus();
        // });

    });

    $(document).on('change', '#id_entity_name', function(){
        var value = this.value;
        $.ajax({
            url: &quot;/project/get-merged-properties/&quot;,
            type: &quot;GET&quot;,
            data: {
                'entity_id': value,
            },
            success: function(response){
                response = JSON.parse(response);
                var pro_options = &quot;&lt;div class='checkbox'>&lt;/div>&quot;;
                pro_options = $(pro_options);
                for (var i=0; i&lt;response.length; i++){
                    var option

                    var input = &quot;&lt;div class='checkbox'>&lt;input type='checkbox' name='property_id' value=&quot; + response[i].id + &quot;>&quot; + &quot;&lt;strong>&quot; + response[i].name + &quot;&lt;/strong>&quot; + &quot; (&quot; + response[i].depreciation_method + &quot;)&quot; + &quot; (&quot; + response[i].type + &quot;)&quot; + &quot;&lt;/div>&quot;;
                    $(pro_options).append(input)
                }
                $(&quot;#append_property_name&quot;).show();
                $(&quot;#append_property_name&quot;).html(pro_options)

            }
        });
    });

    $(document).on('click', '#submit_project', function(e){
        e.preventDefault()
        var id = $('#prop_select_project').val();

        if(id){
            window.location.href = '/project/project-wizard/'+id+'/step2/';
        }
    });

    $(document).on('click', '#submit_project_prop', function(e){
        e.preventDefault()
        var id = $('#select_project_entity').val();

        if(id){
            window.location.href = '/project/project-wizard/'+id+'/step1/';
        }
    });

    function delete_property_confirm(id){
        $(&quot;#id_delete_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_property(&quot; + id + &quot;)&quot;);
        $(&quot;#id_delete_property_modal&quot;).modal(&quot;show&quot;);
    }

    function delete_property(id){
        window.location.replace(&quot;/project/delete-property/&quot;+ id + &quot;/&quot;);
    }

    function delete_project_confirm(id){
        $(&quot;#id_delete_project_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_project(&quot; + id + &quot;)&quot;);
        $(&quot;#id_delete_project_modal&quot;).modal(&quot;show&quot;);
    }

    function delete_project(id){
        window.location.replace(&quot;/project/delete-project/&quot;+ id + &quot;/&quot;);
    }

    function delete_entity_confirm(id){
        $(&quot;#id_delete_entity_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_entity(&quot; + id + &quot;)&quot;);
        $(&quot;#id_delete_entity_modal&quot;).modal(&quot;show&quot;);
    }

    function delete_entity(id){
        window.location.replace(&quot;/project/delete-entity/&quot;+ id + &quot;/&quot;);
    }

    function apply_merge($this) {
        $.ajax({
            url: $this.attr(&quot;action&quot;),
            type: $this.attr(&quot;method&quot;),
            data: $this.serialize(),
            success: function(response) {
                response = JSON.parse(response);
                if(response.status){
                    window.location.href = &quot;project/update_property/&quot; + response.data + &quot;/&quot; + response.property + &quot;/?properties=&quot;+response.property_ids;
                    // take to the edit entity page with prefilled data.
                }else{
                    // show error messages.
                    $('.modal').scrollTop(0,0);
                    $(&quot;#id_merge_error_message&quot;).show();
                    $(&quot;#id_merge_error_message&quot;).html(response.error);
                    }
                }
            });
        return false;
    }



    
        
            var read_only_view_enabled = false;
        

        
        var enable_new_ui = false;
        

      window.intercomSettings = {
        app_id: &quot;xtl1gbtq&quot;,
        subdomain: &quot;DEV&quot;,
        
            user_id: &quot;26_DEV&quot;,
            user_hash: &quot;ce710355dfe0a66a262966565068fca11ecbe0c36650da1185171a8e59003ce9&quot;,
            name: &quot;Ralph Jasmin&quot;,
            email: &quot;ralphninojasmin@yahoo.com&quot;,
            created_at: &quot;1613257274000&quot;,
            organization: &quot;New AGH&quot;,
            username: &quot;RalphNJ&quot;
        
      };
    
    (function(){var w=window;var ic=w.Intercom;if(typeof ic===&quot;function&quot;){ic('reattach_activator');ic('update',w.intercomSettings);}else{var d=document;var i=function(){i.c(arguments);};i.q=[];i.c=function(args){i.q.push(args);};w.Intercom=i;var l=function(){var s=d.createElement('script');s.type='text/javascript';s.async=true;s.src='https://widget.intercom.io/widget/xtl1gbtq';var x=d.getElementsByTagName('script')[0];x.parentNode.insertBefore(s,x);};if(w.attachEvent){w.attachEvent('onload',l);}else{w.addEventListener('load',l,false);}}})();

    
        var storageVersion = '20190607';

        var storage = window.localStorage;

        savedStorageVersion = storage.getItem('storageVersion');
        if (storageVersion != savedStorageVersion){
            storage.setItem('showModal', '');
            storage.setItem('showAddItemModal', '');
            storage.setItem('search_assembly_group_no', '');
            storage.setItem('constructionNumber', '');
            storage.setItem('assemblyNumber', '');
            storage.setItem('toggleCost', '');
            storage.setItem('view', '');
            storage.setItem('NewItemId', '');
            storage.setItem('Xoffset', '');
            storage.setItem('Yoffset', '');
            storage.setItem('ParentId', '');
            storage.setItem('NewCustomAssembly', '');
            storage.setItem('SameItemAgain', '');
            storage.setItem('inputs', '');
            storage.setItem('openProjects', '');
            storage.setItem('openEntities', '');
            storage.setItem('', '');

            storage.setItem('storageVersion', storageVersion);
        }
    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}


    .nav-tabs > li.active > a,
    .nav-tabs > li.active > a:hover,
    .nav-tabs > li.active > a:focus{
        color: #fff;
        background-color: #0747a6;
    }
    tbody{
        cursor: pointer;
    }
    .table-hover tbody tr:hover td, .table-hover tbody tr:hover th {
        background-color: #0747a6;
        color: #fff;
    }

    .has-error .help-block, .has-error .control-label, .has-error .radio, .has-error .checkbox, .has-error .radio-inline, .has-error .checkbox-inline, .has-error.radio label, .has-error.checkbox label, .has-error.radio-inline label, .has-error.checkbox-inline label, .has-error .form-control-feedback, .has-error label{
        color: #a94442;
    }

    input.apple-switch {
  position: relative;
  -webkit-appearance: none;
  outline: none;
  width: 35px !important;
  height: 20px !important;
  background-color: #fff;
  border: 1px solid #D9DADC;
  border-radius: 50px;
  box-shadow: inset -20px 0 0 0 #fff;
}

input.apple-switch:after {
    content: &quot;&quot;;
    position: absolute;
    top: 1px;
    left: 1px;
    background: transparent;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    box-shadow: 2px 4px 6px rgba(0,0,0,0.2);
}

input.apple-switch:checked {
    box-shadow: inset 16px 0 0 0 #a7c7df;
    border-color: #a7c7df;
}

input.apple-switch:checked:after {
  left: 15px;
  box-shadow: -2px 4px 3px rgba(0,0,0,0.05);
}
    .analysis_window_tab:hover{
        cursor: pointer;
    }

    .has-error .form-control, .has-error .form-control:focus{
        border: 1px solid #a94442;
    }
    .avatar_img {
        overflow: hidden;
        text-align: center;
        position: relative;
    }
    .avatar_img > img {
        position: absolute;
        height: 100%;
        -webkit-transform: translateX(-50%);
        -moz-transform: translateX(-50%);
        transform: translateX(-50%);
    }
    #id_item{
        background-color: #fff!important;
    }
    input[type=checkbox] {width:20px; height:17px;}
    .tooltip-inner {
        max-width: 550px;
        font-size: 13px;
        text-align: justify;
    }
    .squeeze_warning_button_border{
        border:solid;
        border-color:red;
    }

    

    tr._GDS_scassemb_child{
    background: #656560;
}

    tr.child_GDS_sc {
        background-color: #979795 !important;
    }

    tr.row_GDS_sc {
        background-color: #979795 !important;
    }
    .AW_GDS_sc {
        background-color: #979795 !important;
        border: 2px solid #979795 !important;
        font-weight: bold;
    }
    .BOX_GDS_sc {
        background-color: #979795 !important;
        border-color: #979795 !important;
    }
    .AW_BORDER_GDS_sc {
        border: 2px solid #979795 !important;
        font-weight: bold;
    }
    .buildings, .multiform-remove {
        /* border: 1px solid gray; */
        /* margin-right: 45px; */
        margin: 0;
        position: relative;
        top: 50%;
        -ms-transform: translateY(-50%);
        transform: translateY(-50%);
    }
    .multiform-remove{
        float:right;
        margin-right: 20px
    }
    div[id$='item_container'] {
        position: relative;
        height: 200px;
        border: solid 1px gray;
        border-radius: 20px;
        margin-top: 20px
    }
    .location_in_use_links div:hover, .asset_in_use_links div:hover{
        text-decoration: underline;
    }
#notification-menu{
  top: 60px;
  right: 0px;
  left: unset;
  width: 460px;
  box-shadow: 0px 5px 7px -1px #c1c1c1;
  padding-bottom: 0px;
  padding: 0px;
}
#notification-menu:before{
  content: &quot;&quot;;
  position: absolute;
  top: -20px;
  right: 12px;
  border:10px solid #343A40;
  border-color: transparent transparent #343A40 transparent;
}
.head{
  padding:5px 15px;
  border-radius: 3px 3px 0px 0px;
}
.footer{
  border-radius: 0px 0px 3px 3px;
}
.head a, .footer a{
  background: gray;
}
.notification-box{
  padding: 10px 10px;
  border-bottom: 1px solid #ddd;
  background: white;
}
.bg-gray{
  background-color: #e7e6f1;
}

.float-right{
    float: right;
    background: gray;
}

.notification-box > div > strong {
    font-weight: bold;
    font-size: 13px;
}

.notification-box > p > strong {
    font-weight: bold;
    font-size: 14px;
}

.notification-box > p{
    margin-top: 0;
    margin-bottom: 5px;
}

.notification-box a{
    background: none;
    padding:0px !important;
    color: blue;
}

.notification-box a:hover {
    background: none;
    padding: 0px !important;
    text-decoration: underline;
}

.text-warning {
    color: #727272;
}

@media (max-width: 640px) {
    .dropdown-menu{
      top: 50px;
      left: -16px;
      width: 290px;
    }
    .nav{
      display: block;
    }
    .nav .nav-item,.nav .nav-item a{
      padding-left: 0px;
    }
    .message{
      font-size: 13px;
    }
}

.profile-pic {
    position: relative;
    font-size: 0.9em;
    color: grey;
    cursor: pointer;
    margin-right: 10px;
}

span.fa-circle {
    position: absolute;
    font-size: 2.3em;
    top: -20px;
    color: #c00;
    right: -3px;
}

span.num {
    position: absolute;
    font-size: 1.2em;
    top: -17px;
    color: #fff;
    right: 2px;
}

.dropdown-menu>li>a{
    display: block;
    padding: 2px 20px;
    clear: both;
    font-weight: 400;
    line-height: 2.6;
    color: #333;
    white-space: nowrap;
    background: #fff;
}

.dropdown-menu>li{
    height: 35px;
}

.dropdown-menu>li>a:hover{
    background-color: #c00;
    color: #fff;
}

.notification_popup {
    /* filter: blur(0px); */
    opacity: 95%;
    transform: scale(0.8);
  /*animation: wiggle 2.5s infinite;*/
}


@keyframes wiggle {
    0% { transform: scale(0.8) translateX(0); }
   25% { transform: scale(0.8) translateX(10px); }
   50% { transform: scale(0.8) translateX(0px); }
   75% { transform: scale(0.8) translateX(-10px); }
  100% { transform: scale(0.8) translateX(0px); }
}

.notification_popup:hover {
  animation: none;
  transform: scale(0.9);
}

.notification_popup .myBar, .notification_popup .myProgress{
    display: block !important;
}

.myBar {
    width: 0%;
    /* height: 30px; */
    height: 5px;
    background-color: white;
}
.myProgress{
    width: 100%;
    height: 5px;
    background: #486fa5;
    padding: 0;
    margin: 0;
}

.notification-container{
    max-height: 365px;
}

.notification_popup .notification-container{
    max-height: 250px;
}

.close-notification-popup {
    background: white;
    /* height: 30px; */
    /* width: 47px; */
    border-radius: 10px;
    /* padding: 5px; */
    width: 20px;
    height: 20px;
    padding-left: 7px;
    cursor: pointer;
}

.notification_popup .close-notification-popup{
    display: block;
}

#dropdown_menu_container:not(.notification_popup) .close-notification-popup{
    display:none;
}

.notification_popup .go-back-to-home{
    display: none;
}



    
        
            
                Toggle navigation
                
                
                
            
            
                
            
        
        
        
            
                 Add Project
                 Merge Multiple Properties
                 Add Custom Assembly
                
                     Manage Algorithm
                     Manage Master Algorithm
                
            
        

        
            
                
                    
                        
                            
                            
                                0
                            
                         Ralph  
                    

                    

                    
                
                
                
            
        
        
    


    
                          
                        
                      
                        
                          
                             Notification
                            x
                          
                      
                      
                          
                      
                      View All
                    

                        
                        
                        
                            
                        
                        Ralph
                    
                    New AGH
                    
                    
                    
                        
                        
                        Notification 0
                        
                        Manage Custom Assembly
                        
                        
                        
                        
                        
                            Manage Org Purpose Description
                            Manage Purpose Description Mapping
                            Manage Location
                            Manage Custom Unit
                            Manage Cost Source
                            Manage Org Asset Class
                            Manage Asset Class Mapping
                            Manage Recovery Period
                            Manage Depreciation Methods
                            Manage Asset Class Suggestion Groups
                            Manage Building System
                        
                        
                        
                        
                        
                        
                        
                        Change Password
                        Logout
                    
                    
                        
                        
                          
                             Management
                          
                      
                        Manage Purpose Description
                        
                        Manage Location
                        
                        Manage Custom Unit
                        
                        Manage Cost Source
                        
                        Manage Depreciation Methods
                        
                        Manage Algorithm
                        Manage Master Algorithm
                    

    
        
    
    
        
        
          

        
        
          
            
              We are Building your Property.
            
            So please grab a coffee and let us do the heavy lifting.
            
                
            
          
        
      
    
    
        
        
          ×
          Your report is being generated. Be patient!
        

        
          ×
          We're working on your Modification, we will begin displaying your Worksheet as soon as we're done processing it...
        
        
    
    
        
            
            
        
    

    


    .panel-group .panel {
        border-radius: 0;
        box-shadow: none;
        border-color: #EEEEEE;
    }

    .panel-default > .panel-heading {
        padding: 0;
        border-radius: 0;
        color: #212121;
        background-color: #FAFAFA;
        border-color: #EEEEEE;
    }

    .panel-title {
        font-size: 14px;
    }

    .panel-title > a {
        display: block;
        padding: 15px;
        text-decoration: none;
    }

    .more-less {
        float: left;
        color: #212121;
    }

    .panel-default > .panel-heading + .panel-collapse > .panel-body {
        border-top-color: #EEEEEE;
    }


    .multiselect-container>li>a>label>input[type=checkbox]{
        position: relative;
        top: 4px;
    }

    .dropdown-menu li{
        height: initial;
    }


    
        

    #location-in-use:hover, #recovery-period-in-use:hover, #algorithm-in-use:hover, .activate{
        color: #0747A6;
        background: #fff;
    }


    
        Dashboard
        DB
    
    
        

        

            
                
                    
                    View by Projects
                    
                
                
                    
                        Projects
                    
                    
                        Entity
                    
                    
                        Property
                    
                
            

            
                
                    
                        
                            Toggle Property View Order
                        
                        
                            
                                Older First
                            
                            
                                Newer First
                            
                        
                    
                
            
            
                 Filter By :
            

            
            
                
                    
                        
                            Project Owner
                        
                        
                           
                                My Projects
                                
                                    Rich
                                
                                    Jonas
                                
                                    deepak
                                
                                    ClarenceP
                                
                                    Tatjana
                                
                                    Carmel
                                
                                    spp_api
                                
                            
                        
                    
                
            
            

            
                
                    
                        
                            Shared Project Status
                        
                        
                           
                                All Projects
                                Personal Projects
                                Shared Projects
                            
                        
                    
                
            
            
                
                    
                        
                            Business Type
                        
                        
                           
                                Select
                                
                                    Fast Food Restaurant Bldg 
                                
                                    Fast Food Restaurant Site 
                                
                                    Medical Office Building 
                                
                                    Medical Office Site 
                                
                                    Motel-Hotel Building 
                                
                                    Motel-Hotel Site 
                                
                                    Multi-Family Residential Building 
                                
                                    Multi-Family Residential Site 
                                
                                    Office Building 
                                
                                    Office Site 
                                
                                    Office Warehouse/Light Mftg Building 
                                
                                    Office Warehouse/Light Mftg Site 
                                
                                    Self-Storage Building 
                                
                                    Self-Storage Site 
                                
                                    Single Family Residence Building 
                                
                                    Single Family Residence Site 
                                
                                    Site Parking Garage 
                                
                                    Small Multi-Family Residential Building 
                                
                                    Small Multi-Family Residential Site 
                                
                                    SPP - Single Family Bldg 
                                
                                    SPP - Single Family Site 
                                
                                    Strip Retail Shopping Center Building 
                                
                                    Strip Retail Shopping Center Site 
                                
                                    uniformat assembly test 
                                
                            
                        
                    
                
            
            
                
                    
                        
                            Property Purchased/New Status
                        
                        
                           
                                Select
                                Purchase Price Allocation (No Given Costs)
                                New Construction / Reno / TI's (Given Costs)
                            
                        
                    
                
            
            
                
                    
                        
                            Zip Code
                        
                        
                           
                        
                    
                
                
                    
                        
                            Reset View/Filters
                        
                    
                
            
        

        
    
    
    



    $(document).on(&quot;change&quot;, &quot;#selected_algorithm_tag&quot;, function(){
        window.location.href = &quot;/algorithm/?id=26_DEV&amp;tag=&quot;+$(this).val();
    });
    $(document).ready(function(){
        $(document).on('change keyup', '.building_types', function(){
            var view = $('input[name=&quot;view_mode&quot;]:checked').val();
            var order = $('input[name=&quot;sort_order&quot;]:checked').val();
            filter_ajax_call(view, order);
        });

        $(document).on('change', '.view_by_input', function(){
            var view = $(this).val();
            $(&quot;#id_select_view&quot;).text($(this).data(&quot;label&quot;))
            var order = $('input[name=&quot;sort_order&quot;]:checked').val();
            filter_ajax_call(view, order);
        });

        $(document).on('click', '#reset_button', function(){
            $('input[name=&quot;sort_order&quot;][value=&quot;oldest&quot;]').prop('checked', true);
            $('input[name=&quot;view_mode&quot;][value=&quot;projects&quot;]').prop('checked', true);
            $('#organization_user').val('')
            $('#building_select').val('');
            $('#property_select').val('');
            $('#zip_code').val('');
            $('#search').val('');
            $('#access_type').val('all');

            var order = $('input[name=&quot;sort_order&quot;]:checked').val();
            var view = $('input[name=&quot;view_mode&quot;]:checked').val();
            $('#label_list_dashboard').html('Project');
            filter_ajax_call(view, order, state=true);
        });

        $(document).on('change', '.sort_order', function(){
            var order = $(this).val();
            var view = $('input[name=&quot;view_mode&quot;]:checked').val();
            filter_ajax_call(view, order);
        });

        $(&quot;.location_in_use_links&quot;).click(function(){
            $(&quot;#location_in_use_clicked&quot;).val($(this).data(&quot;lid&quot;));
            $(&quot;#building_in_use_clicked&quot;).val($(this).data(&quot;buildid&quot;));
            $(&quot;#location_view_mode_form&quot;).submit();
        })

        $(&quot;.asset_in_use_links&quot;).click(function(){
            $(&quot;#asset_in_use_clicked&quot;).val($(this).data(&quot;aid&quot;));
            $(&quot;#asset_view_mode_form&quot;).submit();
        });

        $(&quot;#id_assembly_title_lock_control&quot;).click(function(){
            isAssemblyTitlesLock = $(this).prop(&quot;checked&quot;)
        })

        $(document).on('click', '#location-in-use', function(){

            $('#id_locations_detail').show();
            $('#id_locations_detail').siblings('ul').hide();

            $(this).addClass('activate');
            $(this).siblings().removeClass('activate')
        });

        $(document).on('click', '#algorithm-in-use', function(){
            $('#id_algorithm_detail').show();
            $('#id_algorithm_detail').siblings('ul').hide();

            $(this).addClass('activate');
            $(this).siblings().removeClass('activate')
        });

        $(document).on('click', '#recovery-period-in-use', function(){
            $('#id_recovery_period_detail').show();
            $('#id_recovery_period_detail').siblings('ul').hide();

            $(this).addClass('activate');
            $(this).siblings().removeClass('activate')
        });
    });

    function filter_ajax_call(view, order, state=false){
        var organization_user = $('#organization_user').val();
        var build_value = $('#building_select').val();
        var prop_value = $('#property_select').val();
        var zip_code = $('#zip_code').val();
        var query = $('#search').val();
        var access = $('#access_type').val();
        $('#add_property').hide();
        $('#label_list_dashboard').parent('h4').css('margin-left', '10px');
        $('#label_list_dashboard').html('Project');
        $('#id_processing').show();

        var reset_state = ''

        if (state){
            reset_state = 'reset_state'
        }

        if(view == 'property'){
            $('#add_entity').show();
            $('#add_property').show();
            $('#label_list_dashboard').parent('h4').css('margin-left', '0px');
            $('#label_list_dashboard').html('Property');
        } else if(view == 'entity'){
            $('#add_entity').show();
            $('#label_list_dashboard').parent('h4').css('margin-left', '14px');
            $('#label_list_dashboard').html('Entity');
        }

        $.ajax({
            type: &quot;POST&quot;,
            url: &quot;/&quot;,
            data: {
                'build_id': build_value,
                'organization_user': organization_user,
                'prop_type': prop_value,
                'zip_code': zip_code,
                'q': query,
                'view_mode': view,
                'access_type': access,
                'sort_order': order,
                'reset_state': reset_state,
                &quot;csrfmiddlewaretoken&quot;: &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;
            },
            cache: false,
            success: function(response){
                $('#id_processing').hide();
                $('#project_list_div').html(response.data);

            }
        });

    }


        
            
                Project List
                
                    
                        
                            Search
                            
                            
                        
                    
                
                
                    
                        
                            Add Entity
                        
                        
                            Add Property
                        
                    
                
            
            




    
        
            
                
                    
                           1.  April Test Project   1
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  Test_1   1
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    April 21
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           2.  Pro QA   8
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  April 2021,LLC   26
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    April 5 PPA Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Motel 5April PPA Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Motel-Hotel 5 April
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Motel-Hotel 5April PPA Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Motel-Hotel 5April PPA Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Motel-Hotel 5April PPA  Quantity Changes Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Motel-Hotel 5April PPA Test 2- Select by BUG
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction Template for Bug: New Construction: Direct Cost Setup
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Motel-Hotel 5April PPA  Quantity Changes Test - CLONE
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Testing 8April
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction Template for Bug: New Construction: Direct Cost Setup 1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    GREAT WALL
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    GREAT WALL ONE
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    GREAT WALL TEMPLATED
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    GREAT WALL TEMPLATED 14 April
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    eps Test 19 Apri
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    21april PPA test clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    GREAT WALL 21 April
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    21 April Template Check
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    21 April template PPA Property Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    28 April PPA test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    28 April Hotel Motel
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Merged Property ggg
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    CLONE Merged Property 11MAY
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    28 April Hotel Motel Clone 2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template for QA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               2.  March 2021, LLC   34
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    1March testing
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    2 March Clone PPA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Motel PPA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Merged Property March 3 2021
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    1March testing Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template Check 1March
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template 1Mar testing
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Bug Test 999
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Property For Merging 1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Property For Merging 2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Merged Propert Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Import TEmplate test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS Bug Test. 888
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    10 March PPA Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Validation System into the DA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    15 March testing Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    15 March Clone Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Validation System into the DA Template check
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    DA Validation
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    DA Validation 16MARCH
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    16 March Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    16 March clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    25 March clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    DA Form Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS New Construction Work Flow
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Motel-Hotel Building Module
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    25 March clone 2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    TEmplate New Contruction Property 2962
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS New Construction Work Flow
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    test 45
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    testis_default
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               3.  February 2021, LLC   39
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    PPA Template V2. 2Feb
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    3 Feb Testing
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS Import Previous Asset Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS Import Previous Assets Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    2500 plus lines
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    5000 Property Testing. Don't delete
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS TEsting
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS Asset Name Change Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS Name test to change asset name in DA Form
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Building 6 FEB
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    ASSET NAME TEST
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Building 6 FEB Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    8 Feb Testing
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    9 Feb Testing
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    9 Feb Template Check
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    10 Feb Template Check
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    13 Feb Testing 123
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    2500 plus lines Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    2500 11Feb plus lines Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    2500 plus lines Clone 2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    14Feb PPA Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    16 Feb TEstinG
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Low Record Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Import Values from an Existing Asset: Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Import Values Test 19Feb
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Import Values Bug Test 19Feb
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Save and submit all issue Bug
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Save and Submit Bug Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Server Error Template Check Property Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template Checking an incompletely Setup Property Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Test prop 20Feb
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Bug On DA Form Page for Tenant Module: Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    22 Feb NormalQA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    24 FEB EPS Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    24 FEB MEDICAL BUILDING
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    24 Feb NormalQA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Runaway TEmplate 333
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template Check 24 Feb
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    27 Feb testing 123
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               4.  January 2021, LCC   26
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    PPA Template V2. 1  JanV1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 3 January Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 3 January C1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template check 8Jan
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    9 Jan PPA Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    9 Jan PPA Template Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    9 Jan PPA Template Clone V2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    14 Jan PPA Template Clone V2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    9 Jan PPA Template CHECK.
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    18 Jan Template PPA Test.
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template property Test 21Jan
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    24 Jan PPA Template Clone.
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    24 Jan PPA Template Check
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 24Jan
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 30 Oct
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    25 Jan PPA Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Prop 1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Prop 1 Template Check
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Prop 2 Template check
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Single Family Res Site Module Template Dont Delete
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Single Family Res Site Module Templated
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Single Family Res Site Module Template 28Jan
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 29Jan
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Single Family Res Site Module Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 30
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 31JAN
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               5.  DECEMBER TESTING, LLC   17
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    PPA Template V2 Clone 3 Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2 Clone 5 Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template 5 Dec'S 20
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 8 DEC
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 13 Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template Check B2 15Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template Check V2. 15 DEC
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    8888
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    18 Dec Test Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Job 18 Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hi-Rec-Prop 18D
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hi-Rec-Prop 18D clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 20 Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Site 23Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    HiProb Job 23Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template 23 Dec
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    29 December PPA Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               6.  Enhance Property Setup, LLC   4
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    PPA Template B1. 21 Nov For enhance property Setup
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template B1. Enhance Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. For Template. Don't Delete
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA $Template #V2. For Template. Building's
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               7.  Version 2, LLC   17
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    5 Buildings PPA Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template 2 Buildings V1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Merged Property with 2bldgs
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 3 Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template Nov 4
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Merged Property nov 4
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 6 Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 10NOV
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 10 Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 10 NOV
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2.  10 Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 10 Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Merged Property Nov 11
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 18 Oct - C
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 25 Nov Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 25 Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template V2. 29Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               8.  SegStream Pro LLC   27
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    Hotel Motel PPA Template 2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Motel PPA 2 Building Original Don't Delete
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Motel PPA Template 21Oct
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Motel 2 buildings Template for QA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Template for QA  23Oct20
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template Clone 24Oct
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Motel PPA 5 Building Original Don't Delete>
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    5 Property Duplicate 24Oct20
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Clone V2. 18 Nov
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA CLONE V2. 30 Oct
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    SITE Testing For worksheet
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    TESTING 18 DEC
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    26 Dec Test on EPS v1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction V1 Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Take off Cost Test Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    26 Dec Test on EPS v1.112
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    31Jan LATENCY CHECK
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    5000 Plus Line Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    5001 Plus Line Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    10000 Lines Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction 2April
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction 2April Clone
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Test New Construction 2April
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Construction Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Construction Template 999
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction 6Apr TEmplate
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           3.  QA Test: Algo Refresh [DO NOT MODIFY]   5
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  Test5   1
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    Normal Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               2.  Test4   1
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    Unsqueeze
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               3.  Test3   1
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    Squeeze
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               4.  Test2   1
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    PPA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               5.  Test1   3
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    New Construction[Hotel]
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Building
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    AGH Report Test 041521
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           4.  Test case 1   1
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  test   21
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    Test1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    PPA Template for QA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Construction Template
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    2500 sheet
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    5000 lines
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    QA Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    5000 lines
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS- verification
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    test12
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Video provides a powerful way to help you prove your point. When you click
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Video provides a powerful way to help you prove your point. When you click
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Video provides a powerful way to help you prove your point. When you clone-
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Validation
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    validation 2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Validation on DA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Validation on DA form
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Validation on DA form2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    EPS- verification2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    DA form
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Demo2
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    5000 lines
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           5.  test   1
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  test3   4
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Haven Apartments - Final(restored)
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Office Property - PPA
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    validation
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           6.  Demo   1
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  Test   1
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    demo
                                                    
                                                    
                                                    
                                                    
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           7.  QA Proj   2
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  QA2   1
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    Medical Building
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                        
                            
                                
                                    
                                        
                                               2.  QA1   9
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    QA Demo Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Lem Property
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    New Test
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    MH PPA Ready
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    test3
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    test4
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Test5
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Hotel Demo
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           8.  Test2   1
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  test3   2
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    test4
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    test5
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    

    
        
            
                
                    
                           9.  Test1   1
                        >
                    
                
                 
                 
                 
            
            
                
                    
                        
                            
                                
                                    
                                        
                                               1.  Project 1   3
                                            >
                                        
                                    
                                     
                                     
                                
                                
                                    
                                        
                                            
                                                
                                                    
                                                    tEST123
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Test1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                                
                                                    
                                                    Test1
                                                    
                                                    
                                                    
                                                    
                                                        
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                    
                                                    
                                                    
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                
            
        
    





    var storage = window.localStorage;

    $(document).ready(function () {
        if($(&quot;#search&quot;).val() != &quot;&quot;){
            $(&quot;.search-form .form-group&quot;).addClass('hover');
        } else {
            $(&quot;.search-form .form-group&quot;).removeClass('hover');
        }

        $('[data-toggle=&quot;tooltip&quot;]').tooltip();

        $('#sidebarCollapse').on('click', function () {
            $('#sidebar').toggleClass('active');
            if($('#sidebar').hasClass('active')){
                $(&quot;#content&quot;).removeClass('content-active-block');
                $(&quot;#content&quot;).addClass('content-inactive-block');

            }else{
                $(&quot;#content&quot;).removeClass('content-inactive-block');
                $(&quot;#content&quot;).addClass('content-active-block');
            }
        });

        $('input[type=radio][name=view_mode]').change(function() {
            $(&quot;#view_mode_form&quot;).submit();
        });

        $(document).ready(function(){
            $('[data-toggle=&quot;accordian&quot;]').each(function(){
                $(this).hide();
                var project_type = $(this).attr('data-shared');

            if(project_type == 'owner'){
                $(this).find('.more-less').toggleClass('fa-folder-open fa-folder');
            }else{
                $(this).find('.more-less').toggleClass('shared-folder-open shared-folder');
            }
            $(this).find('.more-less').closest('accordian').removeAttr('style');
            })
        })

        $('#project_list_div [data-toggle=&quot;accordion&quot;]').on('click', function(e) {
            var target = $(this).attr('href');
            $(target).slideToggle(300);
            e.preventDefault();

            var data_type = $(this).attr('data-type');
            var data_type_id = $(this).attr('data-type-id');
            var project_type = $(this).attr('data-shared');

            if(project_type == 'owner'){
                $(this).find('.more-less').toggleClass('fa-folder-open fa-folder');
            }else{
                $(this).find('.more-less').toggleClass('shared-folder-open shared-folder');
            }
            $(this).find('.more-less').closest('accordian').removeAttr('style');
        });

        $('.delete-btn').each(function() {
            $(this).on('click', function(e) {
                e.preventDefault();
                e.stopPropagation();
                var id = $(this).attr('data-project-id');
                $(&quot;#id_delete_project_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_project(&quot; + id + &quot;)&quot;);
                $(&quot;#id_delete_project_modal&quot;).modal(&quot;show&quot;);
            })
        })
        $('.delete-entity').each(function() {
            $(this).on('click', function(e) {
                e.preventDefault();
                e.stopPropagation();
                var id = $(this).attr('data-entity-id');
                $(&quot;#id_delete_entity_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_entity(&quot; + id + &quot;)&quot;);
                $(&quot;#id_delete_entity_modal&quot;).modal(&quot;show&quot;);
            })
        })

        $(&quot;#id_select_project&quot;).change(function () {
            $(&quot;#append_property_name&quot;).html('')
            var value = this.value;
            $.ajax({
                url: &quot;/project/get-merged-enities/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'project_id': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;entity_name&quot; id=&quot;id_entity_name&quot;>\
                        &lt;option value=&quot;&quot;>Select&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].id+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].name  +&quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#append_entities_name&quot;).show();
                    $(&quot;#append_entities_name&quot;).html(select);
                }
            });
        });

        // $('#id_delete_property_modal').on('shown.bs.modal', function(e){
        //     $('#id_delete_confirm').focus();
        // });

        // $('#id_delete_entity_modal').on('shown.bs.modal', function(e){
        //     $('#id_delete_entity_confirm').focus();
        // });

        // $('#id_delete_project_modal').on('shown.bs.modal', function(e){
        //     $('#id_delete_project_confirm').focus();
        // });

    });

    $(document).on('change', '#id_entity_name', function(){
        var value = this.value;
        $.ajax({
            url: &quot;/project/get-merged-properties/&quot;,
            type: &quot;GET&quot;,
            data: {
                'entity_id': value,
            },
            success: function(response){
                response = JSON.parse(response);
                var pro_options = &quot;&lt;div class='checkbox'>&lt;/div>&quot;;
                pro_options = $(pro_options);
                for (var i=0; i&lt;response.length; i++){
                    var option

                    var input = &quot;&lt;div class='checkbox'>&lt;input type='checkbox' name='property_id' value=&quot; + response[i].id + &quot;>&quot; + &quot;&lt;strong>&quot; + response[i].name + &quot;&lt;/strong>&quot; + &quot; (&quot; + response[i].depreciation_method + &quot;)&quot; + &quot; (&quot; + response[i].type + &quot;)&quot; + &quot;&lt;/div>&quot;;
                    $(pro_options).append(input)
                }
                $(&quot;#append_property_name&quot;).show();
                $(&quot;#append_property_name&quot;).html(pro_options)

            }
        });
    });

    function delete_property_confirm(id){
        $(&quot;#id_delete_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_property(&quot; + id + &quot;)&quot;);
        $(&quot;#id_delete_property_modal&quot;).modal(&quot;show&quot;);
    }

    function delete_property(id){
        window.location.replace(&quot;/project/delete-property/&quot;+ id + &quot;/&quot;);
    }

    function delete_project_confirm(id){
        $(&quot;#id_delete_project_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_project(&quot; + id + &quot;)&quot;);
        $(&quot;#id_delete_project_modal&quot;).modal(&quot;show&quot;);
    }

    function delete_project(id){
        window.location.replace(&quot;/project/delete-project/&quot;+ id + &quot;/&quot;);
    }

    function delete_entity_confirm(id){
        $(&quot;#id_delete_entity_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_entity(&quot; + id + &quot;)&quot;);
        $(&quot;#id_delete_entity_modal&quot;).modal(&quot;show&quot;);
    }

    function delete_entity(id){
        window.location.replace(&quot;/project/delete-entity/&quot;+ id + &quot;/&quot;);
    }

    function apply_merge($this) {
        $.ajax({
            url: $this.attr(&quot;action&quot;),
            type: $this.attr(&quot;method&quot;),
            data: $this.serialize(),
            success: function(response) {
                response = JSON.parse(response);
                if(response.status){
                    window.location.href = &quot;project/update_property/&quot; + response.data + &quot;/&quot; + response.property + &quot;/?properties=&quot;+response.property_ids;
                    // take to the edit entity page with prefilled data.
                }else{
                    // show error messages.
                    $('.modal').scrollTop(0,0);
                    $(&quot;#id_merge_error_message&quot;).show();
                    $(&quot;#id_merge_error_message&quot;).html(response.error);
                    }
                }
            });
        return false;
    }



        
        
        
            
                
                    
                        ×
                        Delete Property
                    
                    
                        Are You sure you want to delete this property?
                    
                    
                        
                            Close
                        
                        
                            Delete
                        
                    
                
            
        

        

        
        
            
                
                    
                        ×
                        Delete Project
                    
                    
                        Are You sure you want to delete this project?
                    
                    
                        
                            Close
                        
                        
                            Delete
                        
                    
                
            
        
        
            
                
                    
                        ×
                        Delete Entity
                    
                    
                        Are You sure you want to delete this entity?
                    
                    
                        
                            Close
                        
                        
                            Delete
                        
                    
                
            
        
        
            
                
                    
                        
                            ×
                             Merge Multiple Properties
                            
                            
                            
                                
                                    
                                        Select Properties to be merged:
                                    
                                    
                                        
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       1.  April Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  Test_1
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     April 21
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       2.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       3.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       4.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       5.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       6.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       7.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       8.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       9.  QA Test: Algo Refresh [DO NOT MODIFY]
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  Test1
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction[Hotel]
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Building
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     AGH Report Test 041521
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           2.  Test2
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           3.  Test3
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Squeeze
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           4.  Test4
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Unsqueeze
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           5.  Test5
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Normal Property
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       10.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       11.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       12.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       13.  March Test Project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       14.  March Test project
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       15.  Test case 1
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  test
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Test1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template for QA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     2500 sheet
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     5000 lines
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     QA Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     5000 lines
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS- verification
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test12
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Video provides a powerful way to help you prove your point. When you click
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Video provides a powerful way to help you prove your point. When you click
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Video provides a powerful way to help you prove your point. When you clone-
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Validation
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     validation 2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Validation on DA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Validation on DA form
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Validation on DA form2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS- verification2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     DA form
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Demo2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     5000 lines
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       16.  test
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  test3
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Haven Apartments - Final(restored)
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Office Property - PPA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     validation
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       17.  Demo
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  Test
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     demo
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       18.  Demo
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  Test123
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       19.  QA Demo
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       20.  QA Proj
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  QA1
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     QA Demo Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Lem Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     MH PPA Ready
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test3
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test4
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Test5
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Demo
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           2.  QA2
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Medical Building
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       21.  Test2
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  test3
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test4
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test5
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       22.  Test1
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  Project 1
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     tEST123
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Test1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Test1
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                                
                                                    
                                                        
                                                            
                                                                
                                                                       23.  Pro QA
                                                                
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           1.  SegStream Pro LLC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Motel PPA Template 2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Motel PPA 2 Building Original Don't Delete
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Motel PPA Template 21Oct
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Motel 2 buildings Template for QA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template for QA  23Oct20
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template Clone 24Oct
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Motel PPA 5 Building Original Don't Delete>
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     5 Property Duplicate 24Oct20
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Clone V2. 18 Nov
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA CLONE V2. 30 Oct
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     SITE Testing For worksheet
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     TESTING 18 DEC
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     26 Dec Test on EPS v1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction V1 Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Take off Cost Test Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     26 Dec Test on EPS v1.112
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     31Jan LATENCY CHECK
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     5000 Plus Line Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     5001 Plus Line Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     10000 Lines Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction 2April
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction 2April Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Test New Construction 2April
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Construction Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Construction Template 999
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction 6Apr TEmplate
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           2.  Version 2, LLC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template 2 Buildings V1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Merged Property with 2bldgs
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 3 Nov
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template Nov 4
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Merged Property nov 4
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 6 Nov
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 10 Nov
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 18 Oct - C
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 25 Nov Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 25 Nov
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 29Nov
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           3.  Enhance Property Setup, LLC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template B1. 21 Nov For enhance property Setup
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template B1. Enhance Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. For Template. Don't Delete
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA $Template #V2. For Template. Building's
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           4.  DECEMBER TESTING, LLC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2 Clone 3 Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2 Clone 5 Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template 5 Dec'S 20
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 8 DEC
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 13 Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template Check B2 15Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template Check V2. 15 DEC
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     8888
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     18 Dec Test Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Job 18 Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hi-Rec-Prop 18D
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hi-Rec-Prop 18D clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 20 Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Site 23Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     HiProb Job 23Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template 23 Dec
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     29 December PPA Template
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           5.  January 2021, LCC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 1  JanV1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 3 January Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 3 January C1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template check 8Jan
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     9 Jan PPA Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     9 Jan PPA Template Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     9 Jan PPA Template Clone V2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     14 Jan PPA Template Clone V2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     9 Jan PPA Template CHECK.
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     18 Jan Template PPA Test.
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template property Test 21Jan
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     24 Jan PPA Template Clone.
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     24 Jan PPA Template Check
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 24Jan
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 30 Oct
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     25 Jan PPA Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Prop 1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Prop 1 Template Check
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Prop 2 Template check
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Single Family Res Site Module Template Dont Delete
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Single Family Res Site Module Templated
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Single Family Res Site Module Template 28Jan
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 29Jan
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Single Family Res Site Module Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 30
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 31JAN
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           6.  February 2021, LLC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template V2. 2Feb
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     3 Feb Testing
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS Import Previous Asset Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS Import Previous Assets Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     2500 plus lines
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     5000 Property Testing. Don't delete
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS TEsting
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS Asset Name Change Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS Name test to change asset name in DA Form
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Building 6 FEB
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     ASSET NAME TEST
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Building 6 FEB Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     8 Feb Testing
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     9 Feb Testing
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     9 Feb Template Check
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     10 Feb Template Check
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     13 Feb Testing 123
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     2500 plus lines Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     2500 11Feb plus lines Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     2500 plus lines Clone 2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     14Feb PPA Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     16 Feb TEstinG
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Low Record Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Import Values from an Existing Asset: Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Import Values Test 19Feb
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Import Values Bug Test 19Feb
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Save and submit all issue Bug
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Save and Submit Bug Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Server Error Template Check Property Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template Checking an incompletely Setup Property Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Test prop 20Feb
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Bug On DA Form Page for Tenant Module: Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     22 Feb NormalQA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     24 FEB EPS Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     24 FEB MEDICAL BUILDING
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     24 Feb NormalQA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Runaway TEmplate 333
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template Check 24 Feb
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     27 Feb testing 123
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           7.  March 2021, LLC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     1March testing
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     2 March Clone PPA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Motel PPA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Merged Property March 3 2021
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     1March testing Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template Check 1March
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template 1Mar testing
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Bug Test 999
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Property For Merging 1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Property For Merging 2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Merged Propert Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Import TEmplate test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS Bug Test. 888
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     10 March PPA Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Template Property
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Validation System into the DA
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     15 March testing Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     15 March Clone Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Validation System into the DA Template check
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     DA Validation
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     DA Validation 16MARCH
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     16 March Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     16 March clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     25 March clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     DA Form Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS New Construction Work Flow
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Motel-Hotel Building Module
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     25 March clone 2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     TEmplate New Contruction Property 2962
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     EPS New Construction Work Flow
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     test 45
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     testis_default
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                                    
                                                                        
                                                                            
                                                                                
                                                                                    
                                                                                           8.  April 2021,LLC
                                                                                    
                                                                                
                                                                            
                                                                            
                                                                                
                                                                                    
                                                                                        
                                                                                            
                                                                                                
                                                                                                     April 5 PPA Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Hotel Motel 5April PPA Template
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Motel-Hotel 5 April
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Motel-Hotel 5April PPA Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Motel-Hotel 5April PPA Clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Motel-Hotel 5April PPA  Quantity Changes Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Motel-Hotel 5April PPA Test 2- Select by BUG
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction Template for Bug: New Construction: Direct Cost Setup
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Motel-Hotel 5April PPA  Quantity Changes Test - CLONE
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Testing 8April
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     New Construction Template for Bug: New Construction: Direct Cost Setup 1
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     GREAT WALL
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     GREAT WALL ONE
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     GREAT WALL TEMPLATED
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     GREAT WALL TEMPLATED 14 April
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     eps Test 19 Apri
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     21april PPA test clone
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     GREAT WALL 21 April
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     21 April Template Check
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     21 April template PPA Property Test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     28 April PPA test
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     28 April Hotel Motel
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     Merged Property ggg
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     CLONE Merged Property 11MAY
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     28 April Hotel Motel Clone 2
                                                                                                
                                                                                            
                                                                                        
                                                                                            
                                                                                                
                                                                                                     PPA Template for QA
                                                                                                
                                                                                            
                                                                                        
                                                                                    
                                                                                
                                                                            
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                        
                                    
                                
                            
                            
                                
                                    
                                        *Entity in which you want save merged property
                                    
                                    
                                        
                                            Select
                                            
                                                
                                                
                                                    Test_1
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                    Test1
                                                
                                                    Test2
                                                
                                                    Test3
                                                
                                                    Test4
                                                
                                                    Test5
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                    test
                                                
                                                
                                            
                                                
                                                
                                                    test3
                                                
                                                
                                            
                                                
                                                
                                                    Test
                                                
                                                
                                            
                                                
                                                
                                                    Test123
                                                
                                                
                                            
                                                
                                                
                                                
                                            
                                                
                                                
                                                    QA1
                                                
                                                    QA2
                                                
                                                
                                            
                                                
                                                
                                                    test3
                                                
                                                
                                            
                                                
                                                
                                                    Project 1
                                                
                                                
                                            
                                                
                                                
                                                    SegStream Pro LLC
                                                
                                                    Version 2, LLC
                                                
                                                    Enhance Property Setup, LLC
                                                
                                                    DECEMBER TESTING, LLC
                                                
                                                    January 2021, LCC
                                                
                                                    February 2021, LLC
                                                
                                                    March 2021, LLC
                                                
                                                    April 2021,LLC
                                                
                                                
                                            
                                        
                                    
                                
                            
                        
                        
                            Merge
                            Cancel
                        
                    
                
            
        

        
            
                
                    
                        
                            ×
                             Add Property
                            
                                
                                    Select Projects (select one):
                                    
                                        Select
                                        
                                            April Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            QA Test: Algo Refresh [DO NOT MODIFY]
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test Project
                                        
                                            March Test project
                                        
                                            Test case 1
                                        
                                            test
                                        
                                            Demo
                                        
                                            Demo
                                        
                                            QA Demo
                                        
                                            QA Proj
                                        
                                            Test2
                                        
                                            Test1
                                        
                                            Pro QA
                                        
                                    
                                
                            
                        
                        
                            Add
                            Cancel
                        
                    
                
            
        

        
            
                
                    
                        ×
                         Add Entity
                        
                            
                                Select Projects (select one):
                                
                                    Select
                                    
                                        April Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        QA Test: Algo Refresh [DO NOT MODIFY]
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test Project
                                    
                                        March Test project
                                    
                                        Test case 1
                                    
                                        test
                                    
                                        Demo
                                    
                                        Demo
                                    
                                        QA Demo
                                    
                                        QA Proj
                                    
                                        Test2
                                    
                                        Test1
                                    
                                        Pro QA
                                    
                                
                            
                        
                    
                    
                        Add
                        Cancel
                    
                
            
        
    



    
        Click to return on the top page
    

    
    

    
    
        
            
                ×
                Delete Items/Assemblies with zero quantities
            
            
                Are you sure you want to delete all  the records with zero quantity?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                Delete Items/Assemblies with zero quantities
            
            
                Are you sure you want to delete all  the records with zero quantity?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                Set Quantity of records to zero
            
            
                
                
                
                
                
                
                    Are you sure you want to make the quantities of selected  records to zero?
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Revert Cost factor to 1
            
            
                Are you sure you want to revert the cost factors (location, equipment and labour) of the  selected records?
                Note: Custom Items are not affected by Cost Factors
            
            
                Revert the cost factors and modify selected Items again
                Revert the cost factors
                Cancel
            
        
    


    
    
        
            
                ×
                Confirm Revert Cost factor to 1
            
            
                
                
                    Are you sure you want to revert the cost factors (location, equipment and labour) of the filtered records?
                    Note: Custom Items are not affected by Cost Factors
                
                
                
                
                    Revert the cost factors
                    Revert the cost factors and modify selected Items again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                 Add New Project
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Project Name
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Save
                    Cancel
                
            
        
    



    $(document).on('click', '.add-project-save-button', function(e){
        e.preventDefault();
        var valid = true;

        if (!$('#id_title').val()){
            valid = false
            $('.not-valid-error').html('This field is required.')

        }

        if(valid){
            $('.add-project-form').submit();
        }
    })


    
    
        
            
                ×
                Update multiple Quantity
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Quantity of  selected records.
                                        
                                        
                                            Update Final Quantity
                                            
                                        
                                        
                                            Update Unit Quantity
                                            
                                        
                                        
                                            
                                                Final Quantity: 
                                            
                                            
                                                
                                                
                                            
                                        
                                        
                                            
                                                Unit Quantity: 
                                            
                                            
                                                
                                                
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    



    $(document).on('change', '#unit_quantity', function(e){
        $('#id_final_quantity').hide();
        $('#id_unit_quantity').show();
    });

    $(document).on('change', '#final_quantity', function(e){
        $('#id_final_quantity').show();
        $('#id_unit_quantity').hide();
    });

    $(document).on('keyup', '.quantity_update', function(){

        if($(this).val().length > 7){
            if ($('#id_unit_quantity').is(':visible')) {
                $('.unit-error').html('Invalid input')
                $('.submit_quantity').prop('disabled', true);
            }else{
                $('.final-error').html('Invalid input')
                $('.submit_quantity').prop('disabled', true);
            }
        }else{
            $('.final-error').html('')
            $('.unit-error').html('')
            $('.submit_quantity').prop('disabled', false);
        }

    });


    
    
        
            
                ×
                Update multiple Quantity
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Quantity of  selected records.
                                        
                                        
                                            Update Final Quantity
                                            
                                        
                                        
                                            Update Unit Quantity
                                            
                                        
                                        
                                            
                                                Final Quantity: 
                                            
                                            
                                                
                                            
                                        
                                        
                                            
                                                Unit Quantity: 
                                            
                                            
                                                
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    



    $(document).on('change', '#unit_quantity_filter', function(e){
        $('#id_final_quantity_filter').hide();
        $('#id_unit_quantity_filter').show();
    });

    $(document).on('change', '#final_quantity_filter', function(e){
        $('#id_final_quantity_filter').show();
        $('#id_unit_quantity_filter').hide();
    });


    
    
        
            
                ×
                Modify Global Pricing Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Global Pricing Factor of  selected records.
                                        
                                        
                                            Global Pricing Factor: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Modify Global Pricing Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Global Pricing Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Revert Costs to Original Database Values
            
            
                Are you sure you want to Revert Costs to Original Database Values of  selected records?
            
            
                Update
                Update &amp; Modify Selected Items Again
                Cancel
            
        
    


    
    
        
            
                ×
                Confirm Revert Costs to Original Database Values
            
            
                
                
                    Are you sure you want to Revert Costs to Original Database Values of filtered records?
                
                
                
                
                    Update
                    Update &amp; Modify Selected Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                 Update multiple Concationation
            
            
                
                    
                        
                            
                                
                                    
                                        Custom Items are not available for Purpose Description Prefixes. If you would like to change the description of any Custom Item, you may do so by double-clicking on the record for each Custom Item and changing the description inside of the dialogue that appears. The rest of your selection will be concatenated as desired.
                                    
                                
                            
                        
                    
                
            
            
                Close
            
        
    


    
    
        
            
                ×
                Update multiple Purpose Description Prefix
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Custom Items are not available for Purpose Description Prefixes. If you would like to change the description of any Custom Item, you may do so by double-clicking on the record for each Custom Item and changing the description inside of the dialogue that appears. The rest of your selection will be concatenated as desired.
                                        
                                        
                                            
                                                Update Purpose Description of  selected records.
                                            
                                            
                                                Purpose Description Prefix: 
                                            
                                            
                                                
                                                    None
                                                
                                                    Add Additional Purpose Description Prefix
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                    
                                        
                                            Extra Purpose Description Prefix
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple Purpose Description Prefix
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Custom Items are not available for Purpose Description Prefixes. If you would like to change the description of any Custom Item, you may do so by double-clicking on the record for each Custom Item and changing the description inside of the dialogue that appears. The rest of your selection will be concatenated as desired.
                                        
                                        
                                            
                                                Update Purpose Description of  selected records.
                                            
                                            
                                                Purpose Description Prefix: 
                                            
                                            
                                                
                                                    None
                                                
                                                    Add Additional Purpose Description Prefix
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                    
                                        
                                            Extra Purpose Description Prefix
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Location
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Location of  selected records.
                                        
                                        
                                            Location: 
                                        
                                        
                                            
                                                Select
                                            
                                                Add Additional Location
                                            
                                            
                                            
                                            
                                        
                                    
                                    
                                        
                                            Building
                                        
                                        
                                            
                                                
                                            
                                            
                                        
                                    
                                    
                                        
                                            Extra Location
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                
                
                    ×
                     Import Dynamic Attribute
                
                
                    
                        
                            Upload File
                        
                        
                            
                            
                        
                    
                
            
            
                Upload
                Cancel
            
        
    



    
        
            
                ×
                  Warning
            
            
                
                    
                
            
        
    



    
    
        
            
                
                    ×
                     Import Previous Assets
                    
                    
                        
                            
                                Select Property:
                            
                            
                                
                                    Select
                                    
                                        
                                            
                                                
                                                    
                                                    April 21
                                                    
                                                
                                            
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    New Construction[Hotel]
                                                    
                                                    Building
                                                    
                                                    AGH Report Test 041521
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    PPA
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    Squeeze
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    Unsqueeze
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    Normal Property
                                                    
                                                
                                            
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    Test1
                                                    
                                                    PPA Template for QA
                                                    
                                                    New Construction Template
                                                    
                                                    2500 sheet
                                                    
                                                    5000 lines
                                                    
                                                    QA Test
                                                    
                                                    5000 lines
                                                    
                                                    EPS- verification
                                                    
                                                    test12
                                                    
                                                    Video provides a powerful way to help you prove your point. When you click
                                                    
                                                    Video provides a powerful way to help you prove your point. When you click
                                                    
                                                    Video provides a powerful way to help you prove your point. When you clone-
                                                    
                                                    Validation
                                                    
                                                    validation 2
                                                    
                                                    Validation on DA
                                                    
                                                    Validation on DA form
                                                    
                                                    Validation on DA form2
                                                    
                                                    EPS- verification2
                                                    
                                                    DA form
                                                    
                                                    Hotel Demo2
                                                    
                                                    5000 lines
                                                    
                                                
                                            
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    test
                                                    
                                                    Haven Apartments - Final(restored)
                                                    
                                                    Office Property - PPA
                                                    
                                                    validation
                                                    
                                                
                                            
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    demo
                                                    
                                                
                                            
                                        
                                    
                                        
                                            
                                        
                                    
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    QA Demo Property
                                                    
                                                    Lem Property
                                                    
                                                    Test
                                                    
                                                    New Test
                                                    
                                                    MH PPA Ready
                                                    
                                                    test3
                                                    
                                                    test4
                                                    
                                                    Test5
                                                    
                                                    Hotel Demo
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    Medical Building
                                                    
                                                
                                            
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    test4
                                                    
                                                    test5
                                                    
                                                
                                            
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    tEST123
                                                    
                                                    Test1
                                                    
                                                    Test1
                                                    
                                                
                                            
                                        
                                    
                                        
                                            
                                                
                                                    
                                                    Hotel Motel PPA Template 2
                                                    
                                                    Hotel Motel PPA 2 Building Original Don't Delete
                                                    
                                                    Hotel Motel PPA Template 21Oct
                                                    
                                                    Hotel Motel 2 buildings Template for QA
                                                    
                                                    Template for QA  23Oct20
                                                    
                                                    PPA Template Clone 24Oct
                                                    
                                                    Hotel Motel PPA 5 Building Original Don't Delete>
                                                    
                                                    5 Property Duplicate 24Oct20
                                                    
                                                    PPA Clone V2. 18 Nov
                                                    
                                                    PPA CLONE V2. 30 Oct
                                                    
                                                    SITE Testing For worksheet
                                                    
                                                    TESTING 18 DEC
                                                    
                                                    26 Dec Test on EPS v1
                                                    
                                                    New Construction V1 Test
                                                    
                                                    Take off Cost Test Template
                                                    
                                                    26 Dec Test on EPS v1.112
                                                    
                                                    31Jan LATENCY CHECK
                                                    
                                                    5000 Plus Line Property
                                                    
                                                    5001 Plus Line Property
                                                    
                                                    10000 Lines Property
                                                    
                                                    New Construction Test
                                                    
                                                    New Construction 2April
                                                    
                                                    New Construction 2April Clone
                                                    
                                                    Test New Construction 2April
                                                    
                                                    Construction Template
                                                    
                                                    Construction Template 999
                                                    
                                                    New Construction 6Apr TEmplate
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    PPA Template 2 Buildings V1
                                                    
                                                    Merged Property with 2bldgs
                                                    
                                                    PPA Template V2. 3 Nov
                                                    
                                                    PPA Template Nov 4
                                                    
                                                    Merged Property nov 4
                                                    
                                                    PPA Template V2. 6 Nov
                                                    
                                                    PPA Template V2. 10 Nov
                                                    
                                                    PPA Template V2. 18 Oct - C
                                                    
                                                    PPA Template V2. 25 Nov Clone
                                                    
                                                    PPA Template V2. 25 Nov
                                                    
                                                    PPA Template V2. 29Nov
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    PPA Template B1. 21 Nov For enhance property Setup
                                                    
                                                    PPA Template B1. Enhance Test
                                                    
                                                    PPA Template V2. For Template. Don't Delete
                                                    
                                                    PPA $Template #V2. For Template. Building's
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    PPA Template V2 Clone 3 Dec
                                                    
                                                    PPA Template V2 Clone 5 Dec
                                                    
                                                    PPA Template 5 Dec'S 20
                                                    
                                                    PPA Template V2. 8 DEC
                                                    
                                                    PPA Template V2. 13 Dec
                                                    
                                                    Template Check B2 15Dec
                                                    
                                                    PPA Template Check V2. 15 DEC
                                                    
                                                    8888
                                                    
                                                    18 Dec Test Property
                                                    
                                                    Job 18 Dec
                                                    
                                                    Hi-Rec-Prop 18D
                                                    
                                                    Hi-Rec-Prop 18D clone
                                                    
                                                    PPA Template V2. 20 Dec
                                                    
                                                    Site 23Dec
                                                    
                                                    HiProb Job 23Dec
                                                    
                                                    Template 23 Dec
                                                    
                                                    29 December PPA Template
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    PPA Template V2. 1  JanV1
                                                    
                                                    PPA Template V2. 3 January Clone
                                                    
                                                    PPA Template V2. 3 January C1
                                                    
                                                    Template check 8Jan
                                                    
                                                    9 Jan PPA Template
                                                    
                                                    9 Jan PPA Template Clone
                                                    
                                                    9 Jan PPA Template Clone V2
                                                    
                                                    14 Jan PPA Template Clone V2
                                                    
                                                    9 Jan PPA Template CHECK.
                                                    
                                                    18 Jan Template PPA Test.
                                                    
                                                    Template property Test 21Jan
                                                    
                                                    24 Jan PPA Template Clone.
                                                    
                                                    24 Jan PPA Template Check
                                                    
                                                    PPA Template V2. 24Jan
                                                    
                                                    PPA Template V2. 30 Oct
                                                    
                                                    25 Jan PPA Template
                                                    
                                                    Prop 1
                                                    
                                                    Prop 1 Template Check
                                                    
                                                    Prop 2 Template check
                                                    
                                                    Single Family Res Site Module Template Dont Delete
                                                    
                                                    Single Family Res Site Module Templated
                                                    
                                                    Single Family Res Site Module Template 28Jan
                                                    
                                                    PPA Template V2. 29Jan
                                                    
                                                    Single Family Res Site Module Clone
                                                    
                                                    PPA Template V2. 30
                                                    
                                                    PPA Template V2. 31JAN
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    PPA Template V2. 2Feb
                                                    
                                                    3 Feb Testing
                                                    
                                                    EPS Import Previous Asset Test
                                                    
                                                    EPS Import Previous Assets Test
                                                    
                                                    2500 plus lines
                                                    
                                                    5000 Property Testing. Don't delete
                                                    
                                                    EPS TEsting
                                                    
                                                    EPS Asset Name Change Test
                                                    
                                                    EPS Name test to change asset name in DA Form
                                                    
                                                    Building 6 FEB
                                                    
                                                    ASSET NAME TEST
                                                    
                                                    Building 6 FEB Clone
                                                    
                                                    8 Feb Testing
                                                    
                                                    9 Feb Testing
                                                    
                                                    9 Feb Template Check
                                                    
                                                    10 Feb Template Check
                                                    
                                                    13 Feb Testing 123
                                                    
                                                    2500 plus lines Clone
                                                    
                                                    2500 11Feb plus lines Clone
                                                    
                                                    2500 plus lines Clone 2
                                                    
                                                    14Feb PPA Template
                                                    
                                                    16 Feb TEstinG
                                                    
                                                    Low Record Property
                                                    
                                                    Import Values from an Existing Asset: Test
                                                    
                                                    Import Values Test 19Feb
                                                    
                                                    Import Values Bug Test 19Feb
                                                    
                                                    Save and submit all issue Bug
                                                    
                                                    Save and Submit Bug Test
                                                    
                                                    Server Error Template Check Property Test
                                                    
                                                    Template Checking an incompletely Setup Property Test
                                                    
                                                    Test prop 20Feb
                                                    
                                                    Bug On DA Form Page for Tenant Module: Test
                                                    
                                                    22 Feb NormalQA
                                                    
                                                    24 FEB EPS Test
                                                    
                                                    24 FEB MEDICAL BUILDING
                                                    
                                                    24 Feb NormalQA
                                                    
                                                    Runaway TEmplate 333
                                                    
                                                    Template Check 24 Feb
                                                    
                                                    27 Feb testing 123
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    1March testing
                                                    
                                                    2 March Clone PPA
                                                    
                                                    Hotel Motel PPA
                                                    
                                                    Merged Property March 3 2021
                                                    
                                                    1March testing Clone
                                                    
                                                    Template Check 1March
                                                    
                                                    Template 1Mar testing
                                                    
                                                    Bug Test 999
                                                    
                                                    Property For Merging 1
                                                    
                                                    Property For Merging 2
                                                    
                                                    Merged Propert Test
                                                    
                                                    Import TEmplate test
                                                    
                                                    EPS Bug Test. 888
                                                    
                                                    10 March PPA Test
                                                    
                                                    Template Property
                                                    
                                                    Validation System into the DA
                                                    
                                                    15 March testing Clone
                                                    
                                                    15 March Clone Test
                                                    
                                                    Validation System into the DA Template check
                                                    
                                                    DA Validation
                                                    
                                                    DA Validation 16MARCH
                                                    
                                                    16 March Template
                                                    
                                                    16 March clone
                                                    
                                                    25 March clone
                                                    
                                                    DA Form Test
                                                    
                                                    EPS New Construction Work Flow
                                                    
                                                    EPS Test
                                                    
                                                    Motel-Hotel Building Module
                                                    
                                                    25 March clone 2
                                                    
                                                    TEmplate New Contruction Property 2962
                                                    
                                                    EPS New Construction Work Flow
                                                    
                                                    test
                                                    
                                                    test 45
                                                    
                                                    testis_default
                                                    
                                                
                                            
                                        
                                            
                                                
                                                    
                                                    April 5 PPA Template
                                                    
                                                    Hotel Motel 5April PPA Template
                                                    
                                                    Motel-Hotel 5 April
                                                    
                                                    Motel-Hotel 5April PPA Test
                                                    
                                                    Motel-Hotel 5April PPA Clone
                                                    
                                                    Motel-Hotel 5April PPA  Quantity Changes Test
                                                    
                                                    Motel-Hotel 5April PPA Test 2- Select by BUG
                                                    
                                                    New Construction Template for Bug: New Construction: Direct Cost Setup
                                                    
                                                    Motel-Hotel 5April PPA  Quantity Changes Test - CLONE
                                                    
                                                    Testing 8April
                                                    
                                                    New Construction Template for Bug: New Construction: Direct Cost Setup 1
                                                    
                                                    GREAT WALL
                                                    
                                                    GREAT WALL ONE
                                                    
                                                    GREAT WALL TEMPLATED
                                                    
                                                    GREAT WALL TEMPLATED 14 April
                                                    
                                                    eps Test 19 Apri
                                                    
                                                    21april PPA test clone
                                                    
                                                    GREAT WALL 21 April
                                                    
                                                    21 April Template Check
                                                    
                                                    21 April template PPA Property Test
                                                    
                                                    28 April PPA test
                                                    
                                                    28 April Hotel Motel
                                                    
                                                    Merged Property ggg
                                                    
                                                    CLONE Merged Property 11MAY
                                                    
                                                    28 April Hotel Motel Clone 2
                                                    
                                                    PPA Template for QA
                                                    
                                                
                                            
                                        
                                    
                                
                            
                        
                    
                    
                        
                            
                                Select Asset
                            
                            
                                
                                
                            
                        
                    
                
                
                    Import
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update Location Multiplier
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Location Multiplier of  selected records.
                                        
                                        
                                            Location: 
                                        
                                        
                                            Location Multiplier: 
                                        
                                        
                                        
                                        
                                    
                                    
                                        
                                           
                                            Select
                                            
                                           
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $('#location_list').on('change', function(e){
        var location_modifier = $(this).val();
        var building_id = $('option:selected', this).attr('data-building_id');
        var location = $('option:selected', this).attr('data-location');

        $('#building_location').val(building_id);
        $('#location_id').val(location);
        $('#location_modifier').prop('disabled', false).val(location_modifier);

    })
    $(&quot;#change_location_modifier_btn&quot;).on(&quot;click&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $(&quot;.location_modifier&quot;).each(function(){
            $(this).closest(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);


            if(!$(this).val()){
                $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a value&quot;)
                $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);

                valid=false
            }
            else{
                if(isNaN(parseFloat($(this).val()))){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
                else if(parseFloat($(&quot;.location_modifier&quot;).val()) &lt;= 0){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
            }

        })
        if (valid){
            $(&quot;#location_modifier_form&quot;).submit();
        }
    })

    function location_modifier_clear_validation(){
        $(&quot;.location_modifier_error&quot;).text(&quot;&quot;)
        $(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);
    }


    
    
        
            
                ×
                Map Location
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Locations in use
                                            
                                        
                                        
                                            
                                                Update Location
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Location
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select New Location
                                                
                                                Add Additional Location
                                            
                                            
                                        

                                        
                                            
                                                Building
                                            
                                            
                                                Select
                                                
                                            
                                            
                                        

                                        
                                            Extra Location
                                            
                                            
                                        
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_map_location_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select-location').each(function(){
            if(!$(this).val()){
                $(this).siblings(&quot;.location_error&quot;).text(&quot;Please Select a location&quot;);
                valid=false
            }
        });

        if (valid){
            $(&quot;#update_map_location_form&quot;).submit();
        }
    });

    $(document).on('change', '#location_obj_id', function(e){
        var building_id = $('option:selected', this).attr('data-building_id');

        $('#old_building_id').val(building_id)
    });

    
    
    
        
            
                ×
                Map Takeoff Cost
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Takeoff Cost to be Replaced
                                            
                                        
                                        
                                            
                                                Replace With
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select
                                                
                                                None
                                            
                                            
                                        

                                        
                                            
                                                Select
                                                
                                                None
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#map_tc_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select-tc').each(function(){
            if($(this).val() == null){
                $(this).siblings(&quot;.tc_error&quot;).text(&quot;Please Select a Takeoff cost&quot;);
                valid=false
            }
        });

        if (valid){
            $(&quot;#map_tc_form&quot;).submit();
        }
    });

    
    
        
            
                ×
                Validation Warning!
            
            
                Your Response to this Question is outside of the Recommended Range, what would you like to do?
            
            
                
                    Use Current Response
                
                
                    Edit my Response
                
            
        
    

    
    
        
            
                 ×
                Notification
            
            
                The  Worksheet is Now Ready to Load.

                What would you like to do?
            
            
                Enter the Worksheet Now
                Continue With What I'm Currently Doing
            
        
    

    
    
        
            
                ×
                Map Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Recovery Period in use
                                            
                                        
                                        
                                            
                                                Update Recovery Period
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Recovery Period
                                                
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select New Recovery Period
                                                
                                                Soft Cost
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_recovery_period_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select_recovery_period').each(function(){

            if(!$(this).val()){
                $(this).siblings(&quot;.recovery_period_error&quot;).text(&quot;Please Select Recovery Period&quot;);
                valid=false
            }
        });

        if (valid){
            $(&quot;#update_recovery_period_form&quot;).submit();
        }
    });

    
    
        
            
                ×
                Map Purpose Description to Asset Classes
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Map Purpose Description
                                            
                                        
                                        
                                            
                                                Update Asset Class
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Purpose Description
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select Asset Class
                                                
                                                
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_purpose_to_recovery_period_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select-recovery-period').each(function(){
            if(isNaN($(this).val())){
                $(this).siblings(&quot;.recovery_period_error&quot;).text(&quot;Please Select a value&quot;);
                valid = false;
            }
        });

        if (valid){
            $(&quot;#update_purpose_to_recovery_period&quot;).submit();
        }
    });

    
    
        
            
                ×
                Map Asset Classes to Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Map Asset Classes
                                            
                                        
                                        
                                            
                                                Update Recovery Period
                                            
                                        
                                    
                                    
                                        
                                            
                                                Select Asset Classes
                                                
                                            
                                            
                                        

                                        
                                            
                                                Select Recovery Period
                                                
                                                Indirect Costs
                                            
                                            
                                        
                                        
                                        
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $(document).on(&quot;click&quot;, &quot;#update_asset_class_to_recovery_period_btn&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $('.select_map_class').each(function(){
            if(!$(this).val()){
                $(this).siblings(&quot;.recovery_period_error&quot;).text(&quot;Please Select a value&quot;);
                valid = false;
            }
        });

        if (valid){
            $(&quot;#update_asset_class_to_recovery_period&quot;).submit();
        }
    });

    
    
        
            
                ×
                Bulk Modify
            
            
                
                    
                        
                            
                                
                                    
                                        
                                            Operations to Perform on Selected Lines: 
                                        
                                        
                                            
                                                Clone
                                                Delete
                                                Delete Zero Quantities
                                                Modify Asset Class
                                                Modify Cost Sources
                                                Modify Cost Units
                                                Modify Equipment Cost Factors
                                                Modify Global Pricing Factors (Depreciation / Appreciation)
                                                Modify Labor Cost Factors
                                                Modify Locations
                                                Modify Material Cost Factors
                                                Modify Purpose Description Prefixes
                                                Modify Recovery Period
                                                Modify Takeoff Cost Associations
                                                Modify Takeoff Cost Designations
                                                Modify Quantities
                                                Multiply Locations
                                                Replace Selected Keywords
                                                Revert All Cost Factors to 1.0
                                                Revert Costs to Original Database Values
                                                Zero Out Quantities
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Select
                    Cancel
                
            
        
    


    function open_bulk_tool(){
        tool_selected = $(&quot;#id_select_bulk_tool&quot;).val();
        $('#bulk_tool_select').modal('hide');
        $(&quot;#&quot;+tool_selected).modal(&quot;show&quot;);
        if(tool_selected == 'replace_keywords'){
            search = $(&quot;#bulk-selection-form&quot;).data(&quot;search&quot;);
            $(&quot;#replace_keywords #id_original_keyword&quot;).val(search);
        }
        $(&quot;#bulk-selection-form&quot;).data(&quot;search&quot;, &quot;&quot;)
    }
    $(&quot;#bulk-selection-form-button&quot;).click(function(e){
        e.preventDefault();
        open_bulk_tool();
    });

    $('#id_select_bulk_tool>option').hover(function(e){
        if($(this).is(&quot;:hover&quot;)){
            $(this).css({&quot;background-color&quot;: &quot;rgb(7, 139, 234)&quot;, &quot;color&quot;:&quot;white&quot;, &quot;cursor&quot;: &quot;pointer&quot;});
        }else{
            $(this).css({&quot;background-color&quot;: &quot;white&quot;, &quot;color&quot;: &quot;#111&quot;});
        }
    });


    
    
        
            
                ×
                Bulk Modify
            
            
                
                    
                        
                            
                                
                                    
                                        
                                            Operations to Perform on Selected Lines: 
                                        
                                        
                                            
                                                Clone
                                                Delete
                                                Delete Zero Quantities
                                                Modify Asset Class
                                                Modify Cost Sources
                                                Modify Cost Units
                                                Modify Equipment Cost Factors
                                                Modify Global Pricing Factors (Depreciation / Appreciation)
                                                Modify Labor Cost Factors
                                                Modify Locations
                                                Modify Material Cost Factors
                                                Modify Purpose Description Prefixes
                                                Modify Recovery Period
                                                Modify Takeoff Cost Associations
                                                Modify Takeoff Cost Designations
                                                Modify Quantities
                                                Multiply Locations
                                                Replace Selected Keywords
                                                Revert All Cost Factors to 1.0
                                                Revert Costs to Original Database Values
                                                Zero Out Quantities
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Select
                    Cancel
                
            
        
    


    function open_bulk_filter_tool(){
        tool_selected = $(&quot;#id_select_bulk_filter_tool&quot;).val();
        $('#bulk_tool_select_filter').modal('hide');
        $(&quot;#&quot;+tool_selected).modal(&quot;show&quot;);
        if(tool_selected == 'replace_keywords'){
            search = $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;);
            $(&quot;#replace_keywords #id_original_keyword&quot;).val(search);
        }
        $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;, &quot;&quot;)
    }

    function open_bulk_filter_modify_tool(){
        // tool_selected = $(&quot;#id_select_bulk_filter_tool&quot;).val();
        window.localStorage.setItem(&quot;open_filter_tool&quot;, &quot;true&quot;)
        queryFilteredItems()
        // $('#itemFilter').modal('hide');
        // $('#bulk_tool_select_filter').modal('show');
        // $(&quot;#&quot;+tool_selected).modal(&quot;show&quot;);
        // if(tool_selected == 'replace_keywords'){
        //     search = $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;);
        //     $(&quot;#replace_keywords #id_original_keyword&quot;).val(search);
        // }
        // $(&quot;#bulk-filter-form&quot;).data(&quot;search&quot;, &quot;&quot;)
    }

    $(&quot;#bulk-filter-form-button&quot;).click(function(e){
        e.preventDefault();
        open_bulk_filter_tool();
    });

    $('#id_select_bulk_filter_tool>option').hover(function(e){
        if($(this).is(&quot;:hover&quot;)){
            $(this).css({&quot;background-color&quot;: &quot;rgb(7, 139, 234)&quot;, &quot;color&quot;:&quot;white&quot;, &quot;cursor&quot;: &quot;pointer&quot;});
        }else{
            $(this).css({&quot;background-color&quot;: &quot;white&quot;, &quot;color&quot;: &quot;#111&quot;});
        }
    });


    
    
        
            
                ×
                Update multiple Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Recovery Period of  selected records.
                                        
                                        
                                            Recovery Period: 
                                        
                                        
                                            
                                                
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple Recovery Period
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Recovery Period of  selected records.
                                        
                                        
                                            Recovery Period: 
                                        
                                        
                                            
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update Location Multiplier
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Location Multiplier of  selected records.
                                        
                                        
                                            Location: 
                                        
                                        
                                            Location Multiplier: 
                                        
                                        
                                        
                                        
                                    
                                    
                                        
                                           
                                            Select
                                            
                                           
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Cancel
                
            
        
    


    $('#location_filter_list').on('change', function(e){
        var location_modifier = $(this).val();
        var building_id = $('option:selected', this).attr('data-building_id');
        var location = $('option:selected', this).attr('data-location');

        $('#filter_building_location').val(building_id);
        $('#filter_location_id').val(location);
        $('#filter_location_modifier').prop('disabled', false).val(location_modifier);

    })
    $(&quot;#change_filter_location_modifier_btn&quot;).on(&quot;click&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        var valid = true;

        $(&quot;.location_modifier&quot;).each(function(){
            $(this).closest(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);


            if(!$(this).val()){
                $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a value&quot;)
                $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);

                valid=false
            }
            else{
                if(isNaN(parseFloat($(this).val()))){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
                else if(parseFloat($(&quot;.location_modifier&quot;).val()) &lt;= 0){
                    $(this).siblings(&quot;.location_modifier_error&quot;).text(&quot;Please enter a positive number&quot;)
                    $(this).closest(&quot;.location_modifier_div&quot;).addClass(&quot;has-error&quot;);
                    valid=false
                }
            }

        })
        if (valid){
            $(&quot;#filter_location_modifier_form&quot;).submit();
        }
    })

    function location_modifier_clear_validation(){
        $(&quot;.location_modifier_error&quot;).text(&quot;&quot;)
        $(&quot;.location_modifier_div&quot;).removeClass(&quot;has-error&quot;);
    }


    
    
        
            
                ×
                Update multiple Asset Class
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Asset class of  selected records.
                                        
                                        
                                            Asset Class: 
                                        
                                        
                                            
                                                Select
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                
                
                    ×
                     Import Costs Setup
                
                
                    
                        
                            Upload File
                        
                        
                            
                        
                    
                
            
            
                Upload
                Cancel
            
        
    


    
    
        
            
                ×
                Update Material Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Material Cost of  selected records.
                                        
                                        
                                            Material Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Material Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Material Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Equipment Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Equipment Cost of  selected records.
                                        
                                        
                                            Equipment Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Equipment Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Equipment Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Replace Keyword
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Replace Keyword of  selected records.
                                        
                                        
                                            Keyword: 
                                        
                                        
                                            
                                        
                                    
                                    
                                        
                                            Replace By: 
                                        
                                        
                                            
                                        
                                    
                                    
                                    
                                
                            
                        
                    
                    Note: This tool will not search or modify text located inside of Purpose Description Prefixes.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Replace Keyword
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Keyword: 
                                        
                                        
                                            
                                        
                                    
                                    
                                        
                                            Replace By: 
                                        
                                        
                                            
                                        
                                    
                                    
                                    
                                    
                                
                            
                        
                    
                    Note: This tool will not search or modify text located inside of Purpose Description Prefixes.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Labour Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Labour Cost Factor of  selected records.
                                        
                                        
                                            Labour Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
            
            
                Update
                Update &amp; Modify Same Items Again
                Cancel
            
        
    



    $(document).ready(function(){
        $(document).on('click', '.button_submit', function(e){
            e.preventDefault();
            var valid = true;
            if (!$('#id_updated_labour_cost').val() || !($('#id_updated_labour_cost').val().length &lt; 8)){
                var valid = false

                $('#labour_cost_invalid').text('Invalid input')
            }
            if (valid){
                $('#labour_cost_update_form').submit();
            }
        })
    });


    
    
        
            
                ×
                Update Labour Cost Factor
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Labour Cost Factor: 
                                        
                                        
                                            
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Custom Items are not affected by Cost Factors
                
            
            
                Update
                Update &amp; Modify Same Items Again
                Cancel
            
        
    



    $(document).ready(function(){
        $(document).on('click', '.button_filter_submit', function(e){
            e.preventDefault();
            var valid = true;
            if (!$('#id_filter_updated_labour_cost').val() || !($('#id_filter_updated_labour_cost').val().length &lt; 8)){
                var valid = false

                $('#labour_cost_invalid_filter').text('Invalid input')
            }
            if (valid){
                $('#labour_cost_filter_update_form').submit();
            }
        })
    });


    
    
        
            
                ×
                Designate/Undesignate as Takeoff Cost
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Designate/Undesignate  selected records as Takeoff Cost.
                                        
                                        
                                            Takeoff Cost: 
                                        
                                        
                                            
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Only Custom Items can be designated as Takeoff Costs.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Designate/Undesignate as Takeoff Cost
            
            
                
                
                    
                        
                            
                                
                                    

                                        
                                            Takeoff Cost: 
                                        
                                        
                                            
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                    Note: Only Custom Items can be designated as Takeoff Costs.
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Cost Source
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Cost Source of  selected records.
                                        
                                    
                                    
                                        
                                            Cost Source: 
                                        
                                        
                                            
                                                    Select
                                                
                                                    Contractor Cost
                                                
                                                    Client Cost
                                                
                                                    RS Means
                                                
                                                    Site Contractor Cost
                                                
                                                    Site Indirect
                                                
                                                    Client Cost - Expenses
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple Cost Source
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Cost Source: 
                                        
                                        
                                            
                                                    Select
                                                
                                                    Contractor Cost
                                                
                                                    Client Cost
                                                
                                                    RS Means
                                                
                                                    Site Contractor Cost
                                                
                                                    Site Indirect
                                                
                                                    Client Cost - Expenses
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple unit
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Cost Unit of  selected records.
                                        
                                        
                                            Unit: 
                                        
                                        
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update multiple unit
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Cost Unit of  selected records.
                                        
                                        
                                            Unit: 
                                        
                                        
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Asset Class
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Asset class of  selected records.
                                        
                                        
                                            Asset Class: 
                                        
                                        
                                            
                                                Select
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    

    
    
        
            
                ×
                Update multiple Location
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Location: 
                                        
                                        
                                            
                                                Select
                                            
                                                Add Additional Location
                                            
                                            
                                            
                                            
                                        
                                    
                                    
                                        
                                            Building
                                        
                                        
                                            
                                                
                                            
                                            
                                        
                                    
                                    
                                        
                                            Extra Location
                                        
                                        
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Clone
            
            
                
                
                    
                            
                                
                                    
                                        
                                            
                                                Clone  selected records.
                                            
                                            
                                                Number of Clones to Add: 
                                            
                                            
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                
                            
                        
                        
                            Note:
                                
                                    Cloned Items will be added as new components and will not appear alongside the original component when Viewed by Input Order (or Reverse Input Order).
                                
                            
                        
                
                
                    Clone
                    Clone &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Confirm Clone
            
            
                
                
                    
                            
                                
                                    
                                        
                                            
                                                Number of Clones to Add: 
                                            
                                            
                                                
                                                
                                                
                                                
                                            
                                        
                                    
                                
                            
                        
                        
                            Note:
                                
                                    Cloned Items will be added as new components and will not appear alongside the original component when Viewed by Input Order (or Reverse Input Order).
                                
                            
                        
                
                
                    Clone
                    Clone &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Takeoff Cost Association
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Update Takeoff Cost Association of  selected records.
                                        
                                        
                                            Select Takeoff Cost: 
                                        
                                        
                                            
                                                None
                                                
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                Update Takeoff Cost Association
            
            
                
                
                    
                        
                            
                                
                                    
                                        
                                            Select Takeoff Cost: 
                                        
                                        
                                            
                                                None
                                                
                                            
                                            
                                            
                                            
                                        
                                    
                                
                            
                        
                    
                
                
                    Update
                    Update &amp; Modify Same Items Again
                    Cancel
                
            
        
    


    
    
        
            
                ×
                 Change Password
            
            
                
                
                    
                        
                            
                                Old Password
                            
                            
                                
                                
                            
                        
                        
                            
                                New Password
                            
                            
                                
                                
                            
                        
                        
                            
                                Confirm Password
                            
                            
                                
                                
                            
                        
                    
                
                
                    Save
                    Close
                
            
        
    


    
    
        
                  Please Select atleast one Item
            ×
        
    



    
        
                  No line with zero quantity
            ×
        
    


    

    
        
            
                ×
                 Add Takeoff or Costs
            
            
                
                     Add Normal Items/Assemblies
                     Add Costs/Custom Items
                    
                     Add Custom Assembly Wizard
                
                
                    
                        
                            
                                
                                    
                                        
                                        
                                            
                                                
                                                    
                                                        
                                                            
                                                            Add Items
                                                        
                                                        
                                                            
                                                            Add Assemblies
                                                        
                                                        
                                                            
                                                            Both
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            
                                                            Normal Search
                                                        
                                                        
                                                            
                                                            Favorites Filter
                                                        
                                                    
                                                
                                            

                                            
                                                
                                                    
                                                        
                                                            Construction Division Filter
                                                        
                                                        
                                                            Select
                                                            0. Custom Item
                                                            
                                                                1. General Requirements 
                                                            
                                                                2. Existing Conditions 
                                                            
                                                                3. Concrete 
                                                            
                                                                4. Masonry 
                                                            
                                                                5. Metals 
                                                            
                                                                6. Wood, Plastics, and Composites 
                                                            
                                                                7. Thermal and Moisture Protection 
                                                            
                                                                8. Openings 
                                                            
                                                                9. Finishes 
                                                            
                                                                10. Specialties 
                                                            
                                                                11. Equipment 
                                                            
                                                                12. Furnishings 
                                                            
                                                                13. Special Construction 
                                                            
                                                                14. Conveying Equipment 
                                                            
                                                                21. Fire Suppression 
                                                            
                                                                22. Plumbing 
                                                            
                                                                23. Heating, Ventilating, and Air Conditioning (HVAC) 
                                                            
                                                                26. Electrical 
                                                            
                                                                27. Communications 
                                                            
                                                                28. Electronic Safety and Security 
                                                            
                                                                31. Earthwork 
                                                            
                                                                32. Exterior Improvements 
                                                            
                                                                33. Utilities 
                                                            
                                                                34. Transportation 
                                                            
                                                                35. Waterway and Marine Construction 
                                                            
                                                                41. Material Processing and Handling Equipment 
                                                            
                                                                44. Pollution and Waste Control Equipment 
                                                            
                                                                50. Landscaping Equipent 
                                                            
                                                                46. Water and Wastewater Equipment 
                                                            
                                                                48. Electrical Power Generation 
                                                            
                                                        
                                                    
                                                    

                                                    
                                                
                                                
                                                    
                                                        Assembly Group Filter
                                                        
                                                            Select
                                                            0. Custom Assembly
                                                            
                                                                1. Standard Foundations
                                                            
                                                                2. Special Foundations
                                                            
                                                                3. Slab on Grade
                                                            
                                                                4. Basement Excavation
                                                            
                                                                5. Basement Walls
                                                            
                                                                6. Floor Construction
                                                            
                                                                7. Roof Construction
                                                            
                                                                8. Exterior Walls
                                                            
                                                                9. Exterior Windows
                                                            
                                                                10. Exterior Doors
                                                            
                                                                11. Roof Coverings
                                                            
                                                                12. Roof Openings
                                                            
                                                                13. Partitions
                                                            
                                                                14. Interior Doors
                                                            
                                                                15. Fittings
                                                            
                                                                16. Stair Construction
                                                            
                                                                17. Wall Finishes
                                                            
                                                                18. Floor Finishes
                                                            
                                                                19. Ceiling Finishes
                                                            
                                                                20. Elevators and Lifts
                                                            
                                                                21. Escalators and Moving Walks
                                                            
                                                                22. Other Conveying Systems
                                                            
                                                                23. Plumbing Fixtures
                                                            
                                                                24. Domestic Water Distribution
                                                            
                                                                25. Rain Water Drainage
                                                            
                                                                26. Other Plumbing Systems
                                                            
                                                                27. Energy Supply
                                                            
                                                                28. Heat Generating Systems
                                                            
                                                                29. Cooling Generating Systems
                                                            
                                                                30. Terminal &amp; Package Units
                                                            
                                                                31. Sprinklers
                                                            
                                                                32. Standpipes
                                                            
                                                                33. Other Fire Protection Systems
                                                            
                                                                34. Electrical Service/Distribution
                                                            
                                                                35. Lighting and Branch Wiring
                                                            
                                                                36. Communications and Security
                                                            
                                                                37. Other Electrical Systems
                                                            
                                                                38. Commercial Equipment
                                                            
                                                                39. Institutional Equipment
                                                            
                                                                40. Vehicular Equipment
                                                            
                                                                41. Other Equipment
                                                            
                                                                42. Fixed Furnishings
                                                            
                                                                43. Moveable Furnishings
                                                            
                                                                44. Special Structures
                                                            
                                                                45. Integrated Construction
                                                            
                                                                46. Special Construction Systems
                                                            
                                                                47. Special Facilities
                                                            
                                                                48. Site Earthwork
                                                            
                                                                49. Roadways
                                                            
                                                                50. Parking Lots
                                                            
                                                                51. Pedestrian Paving
                                                            
                                                                52. Site Development
                                                            
                                                                53. Water Supply
                                                            
                                                                54. Sanitary Sewer
                                                            
                                                                55. Storm Sewer
                                                            
                                                                56. Fuel Distribution
                                                            
                                                                57. Site Lighting
                                                            
                                                        
                                                    
                                                    

                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    Favorites Item/Assembly
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            Items Search Result
                                                            
                                                            
                                                        
                                                        
                                                            Assemblies Search Result
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                    
                                                    
                                                        View/Modify Assembly
                                                    
                                                
                                            
                                            

                                            

                                            
                                                
                                                    
                                                        
                                                            Quantity
                                                        

                                                        
                                                            
                                                            
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            Recovery Period
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            
                                                                Purpose Description
                                                            
                                                        
                                                        
                                                        
                                                            
                                                                    Select
                                                                
                                                                    Add Additional Purpose Description
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Purpose Description
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            Location
                                                        
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                                    Add Additional Location
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Building
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Location
                                                        
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            Asset Class
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                            

                                            
                                                
                                                    
                                                        
                                                            
                                                                Takeoff Cost
                                                            
                                                        
                                                        
                                                            
                                                                Select
                                                                
                                                            
                                                            
                                                        
                                                    

                                                    
                                                        
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #1
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #2
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #3
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #4
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                            
                                                                
                                                                    
                                                                        TC #5
                                                                    
                                                                
                                                                
                                                                    
                                                                        Select
                                                                        
                                                                    
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            
                                                                Current Deprecation Method
                                                            
                                                        
                                                        
                                                            
                                                              
                                                              
                                                                None - Replacement Cost New
                                                              
                                                            
                                                            
                                                              
                                                              
                                                               Choose from below
                                                              
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            
                                                                
                                                                    Condition of Property
                                                                
                                                            
                                                            
                                                              
                                                                  Excellent - 90%
                                                                  Good - 75%
                                                                  Fair - 50%
                                                                  Bad - 25%
                                                                  Replacement Ready - 10%
                                                                  Add Custom Condition
                                                              
                                                            
                                                        
                                                        
                                                            
                                                                
                                                                    Custom Condition of Property
                                                                
                                                            
                                                            
                                                                
                                                            
                                                             . 
                                                            
                                                                
                                                            
                                                            
                                                                
                                                                 % 
                                                            
                                                            
                                                        
                                                        
                                                            
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                            
                                            
                                                
                                                
                                                
                                                    Save and add another Item/Assembly
                                                    Save and Add Same Item/Assembly Again
                                                    Save
                                                    Close
                                                    
                                                
                                            
                                        
                                        
                                        
                                    
                                
                            
                        
                        
                            
                            
                                
                                    
                                        
                                        
                                            
                                                
                                                    
                                                        *Description
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        *Unit Cost
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Takeoff Cost
                                                        
                                                            
                                                            

                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Quantity
                                                        
                                                            
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Total Cost
                                                        
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        Cost Source
                                                        
                                                            
                                                                Select
                                                                
                                                                    Contractor Cost
                                                                
                                                                    Client Cost
                                                                
                                                                    RS Means
                                                                
                                                                    Site Contractor Cost
                                                                
                                                                    Site Indirect
                                                                
                                                                    Client Cost - Expenses
                                                                
                                                                Add Additional Cost Source
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Cost Source
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Cost Unit
                                                        
                                                            
                                                                Select
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    EA
                                                                
                                                                    EA
                                                                
                                                                    EA
                                                                
                                                                    EA
                                                                
                                                                    EA
                                                                
                                                                    EA
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    TOT
                                                                
                                                                    TOT
                                                                
                                                                    TOT
                                                                
                                                                    TOT
                                                                
                                                                    TOT
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    SF
                                                                
                                                                    SF
                                                                
                                                                    SF
                                                                
                                                                    SF
                                                                
                                                                    SF
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    LF
                                                                
                                                                    LF
                                                                
                                                                    LF
                                                                
                                                                    LF
                                                                
                                                                    LF
                                                                
                                                                    LF
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    COST1
                                                                
                                                                    TEST1
                                                                
                                                                    Rou-T
                                                                
                                                                    Rou-S
                                                                
                                                                    $%!%
                                                                
                                                                    Rou-T
                                                                
                                                                    TestA
                                                                
                                                                    TST
                                                                
                                                                    TEST
                                                                
                                                                    Test8
                                                                
                                                                    EA2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    tst2
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    tst2
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    tst2
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    TST
                                                                
                                                                    Test8
                                                                
                                                                    EA2
                                                                
                                                                    RS-89
                                                                
                                                                    Rou-T
                                                                
                                                                    8tv3
                                                                
                                                                    AE
                                                                
                                                                    88I
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    COST1
                                                                
                                                                    TEST1
                                                                
                                                                    Rou-T
                                                                
                                                                    Rou-S
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    COST1
                                                                
                                                                    TEST1
                                                                
                                                                    Rou-T
                                                                
                                                                    Rou-S
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    tst2
                                                                
                                                                    SFF
                                                                
                                                                    EA
                                                                
                                                                    TOT
                                                                
                                                                    SF
                                                                
                                                                    LF
                                                                
                                                                    EA2
                                                                
                                                                    TGIF
                                                                
                                                                    TGIF
                                                                
                                                                Add Additional Cost Unit
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Extra Cost Unit
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Location
                                                        
                                                            
                                                                Select
                                                                
                                                                    Add Additional location
                                                            
                                                            
                                                        
                                                    
                                                    
                                                        Building
                                                        
                                                            Select
                                                            
                                                        
                                                        
                                                    
                                                    
                                                        
                                                            Extra Location
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        Recovery Period
                                                        
                                                            
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        Asset Class
                                                        
                                                            
                                                                Select
                                                                
                                                            
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        RS Means numbering
                                                        
                                                            Only by construction division
                                                            Both by construction divison and subgroup
                                                        
                                                    
                                                    
                                                        Item Construction Division /SubGroup Search
                                                        
                                                        
                                                            
                                                            
                                                                
                                                                    
                                                                        Items Search Result
                                                                        
                                                                        
                                                                    
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            *Construction Division
                                                            
                                                                
                                                                    Select
                                                                    
                                                                        1. General Requirements 
                                                                    
                                                                        2. Existing Conditions 
                                                                    
                                                                        3. Concrete 
                                                                    
                                                                        4. Masonry 
                                                                    
                                                                        5. Metals 
                                                                    
                                                                        6. Wood, Plastics, and Composites 
                                                                    
                                                                        7. Thermal and Moisture Protection 
                                                                    
                                                                        8. Openings 
                                                                    
                                                                        9. Finishes 
                                                                    
                                                                        10. Specialties 
                                                                    
                                                                        11. Equipment 
                                                                    
                                                                        12. Furnishings 
                                                                    
                                                                        13. Special Construction 
                                                                    
                                                                        14. Conveying Equipment 
                                                                    
                                                                        21. Fire Suppression 
                                                                    
                                                                        22. Plumbing 
                                                                    
                                                                        23. Heating, Ventilating, and Air Conditioning (HVAC) 
                                                                    
                                                                        26. Electrical 
                                                                    
                                                                        27. Communications 
                                                                    
                                                                        28. Electronic Safety and Security 
                                                                    
                                                                        31. Earthwork 
                                                                    
                                                                        32. Exterior Improvements 
                                                                    
                                                                        33. Utilities 
                                                                    
                                                                        34. Transportation 
                                                                    
                                                                        35. Waterway and Marine Construction 
                                                                    
                                                                        41. Material Processing and Handling Equipment 
                                                                    
                                                                        44. Pollution and Waste Control Equipment 
                                                                    
                                                                        50. Landscaping Equipent 
                                                                    
                                                                        46. Water and Wastewater Equipment 
                                                                    
                                                                        48. Electrical Power Generation 
                                                                    
                                                                Select
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            *Item SubGroup
                                                            
                                                            
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            Backout this Takeoff from this Cost
                                                        
                                                        
                                                            Select
                                                            
                                                        
                                                    
                                                    
                                                
                                            
                                            
                                                
                                                    Save Custom Item &amp; Make Available for Future Use
                                                
                                                
                                                    Save and add another custom item
                                                    Save
                                                    Close
                                                    Reset
                                                    Show Others Fields
                                                
                                            
                                            
                                                
                                                    
                                                        
                                                            PurposeDescription
                                                            
                                                                
                                                                    Select
                                                                    
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            Labour Cost
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Material Cost
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            Equipment Cost
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                                
                                                    
                                                        
                                                            CF2
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            CF
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                    
                                                        
                                                            CF4
                                                            
                                                                
                                                                
                                                            
                                                        
                                                    
                                                
                                            
                                        
                                        
                                        
                                    
                                
                            
                        
                        
                    
                
            
        
    


    .multiselect-container>li>a>label>input[type=radio]{
        position: relative;
        top: 4px;
    }
    .dropdown-menu>li {
        height: 45px;
    }



    

    $(&quot;[name='rsmeans_numbering']&quot;).on(&quot;change&quot;, function(){
        if($(this).val() == '1'){
            $(&quot;#id_item_sub_group_no_error_parent_div&quot;).hide();
            $(&quot;#custom_item_search_box&quot;).hide();
        }
        else{
            $(&quot;#id_item_sub_group_no_error_parent_div&quot;).show();
            $(&quot;#custom_item_search_box&quot;).show();
        }
    });

    

    $(document).ready(function(){
        $(&quot;#id_item_group_no&quot;).select2({
             dropdownParent: $('#add_item_assembly'),
             width: 'resolve'
        });

        $(&quot;#id_item_group_no&quot;).on('select2:open', function(){
            select2_open = true;
        });

        $(&quot;#id_item_group_no&quot;).on('select2:close', function(){
            select2_open = false;
        });

        

    })

    
        $(document).on('change', '#id_purpose_description', function(){
            var check = $('#hidden_value_for_check').val();
            var value = $(this).val();
            console.log(check, 'here')
            if (check &amp;&amp; value &amp;&amp; value != &quot;Others&quot;){
                $.ajax({
                    type: 'GET',
                    url: '/project/map-recovery-period-and-group/',
                    data:{
                        'concat_id': value,
                        'property_id': '',
                    },
                    success: function(response){

                        if(response){

                            $('#id_recovery_period option[value=&quot;'+response.recovery_code+'&quot;]').prop('selected','selected');
                            
                            $('#id_asset_class2').val(response.asset_id);
                            
                        }
                    }

                });
            }

        });

    


    
    
        
            
                ×
                  Basic KeyBoard Shortcuts
            
            
                
                    
                        
                            Keyboard
                            Functionality
                        
                    
                    
                        
                            Shift + i
                            
                                Show/Hide Project Information
                            
                        
                        
                            Shift + w
                            
                                Show Analysis Window
                            
                        
                        
                            Shift + z
                            
                                Add Items/Assemblies
                            
                        
                        
                            Shift + x
                            
                                Add Custom Item
                            
                        
                        
                            Shift + r
                            
                                Add Custom Assembly
                            
                        
                        
                            Shift + s
                            
                                Show Advanced Bulk Select
                            
                        
                        
                            Shift + m
                            
                                Show Advanced Modify Modal
                            
                        
                        
                            Shift + g
                            
                                Close Dialouge Box
                            
                        
                        
                            Shift + e
                            
                                Show/hide the related items
                            
                        
                        
                            Shift + t
                            
                                Toggle Costs.
                            
                        
                        
                            Shift + f
                            
                                Toggle Additional Record Information.
                            
                        
                        
                            Shift + v
                            
                                Toggle between all views.
                            
                        
                        
                            Shift + a
                            
                                Select all Items
                            
                        
                        
                            Esc
                            
                                Deselect all Items
                            
                        
                        
                            Shift + d
                            
                                Delete selected Items
                            
                        
                        
                            Shift + c
                            
                                Update selected Purpose Description Prefix
                            
                        
                        
                            Shift + l
                            
                                Update selected Location
                            
                        
                        
                            Shift + y
                            
                                Update selected Asset year
                            
                        
                        
                            Shift + h
                            
                                Open keyboard shortcuts.
                            
                        
                    
                
            
            
                Close
            
        
    


    
    
        
            
                ×
                  Warning
            
            
                
                    You have selected a template that is of one Property Type, the PPA Property Type, and you have selected to use a different Property Type for this property, the New Construction Property Type.
                    
                    Note: This is allowed but you should understand that there will be some added complexity involved by making this selection.
                    
                    PPA Jobs are built without Direct Cost Data from the Contractor(s) who built the property and, as a result, these jobs do not include any Direct Costs (entered as &quot;Custom Items&quot; within the application).
                    
                    The complexity introduced here, by using a template with this Property Type for this property, is that no Direct Contract Costs from this property currently exist inside of this Worksheet but there are many items and assemblies entered as takeoff. In order for the application to work optimally, all of these items and assemblies must be associated as being &quot;backed out of&quot; a particular Takeoff Cost.
                    
                    First, you will need to input the Direct Costs associated with your property and designate one or more of them to be Takeoff Costs from which the value of the items and assemblies will be deducted. Once the Takeoff Costs have been entered, you can begin to associate each of the items and assemblies with a specific Takeoff Cost from within each item's/assembly's details dialogue by double left-clicking on each record one at a time.
                    
                    Note: Once you have inputted the Takeoff Costs and designated a Takeoff Cost for each cost for each item/assembly, you should now see that the Initial Total Cost and the Active Total Cost are identical. If these numbers are not the same then make sure you have accounted for every single item and assembly being associated with a takeoff cost.
                
            
        
    


    
    
        
            
                ×
                  Warning
            
            
                
                    You have selected a template that is of one Property Type, the New Construction Property Type and you have selected to use a different Property Type with this property, the PPA Property Type.
                    
                    Note: This is allowed but you should understand that there will be some added complexity involved by making this selection.
                    
                    New Construction Property Type Jobs are built around having entered Direct Costs (entered as &quot;Custom Items&quot; within the application) from the Contractor(s) who built the property and then backing out any takeoff of items and assemblies from a specifically designated subset of these Direct Costs, known as Takeoff Costs within the application. This is done so that the value of these takeoffs will be auto-subtracted from these Takeoff Costs and the value of the entire property will remain unchanged throughout the job after all Direct Costs are entered.
                    
                    The complexity introduced here, by using a template with this Property Type for this property, is that there will exist Direct Contract Costs from an entirely unrelated property inside this Worksheet and some of these, those designated as Takeoff Costs, will be associated directly with the items and assemblies entered as takeoff for the property.
                    
                    The easiest solution to get rid of the complexity is just to select and delete all of the Direct Costs in the Worksheet. This is done by selecting each of the Custom Items (by Shift or CTRL left-click) and then using the Delete Command (Shift-D) or by using the Delete Button found in the Additional Controls menu at the top of the Dashboard screen once inside the job.
                    
                    All that will be left in this case will be items and assemblies that were entered from the RS Means database. Of course, if you need to cull additional items or assemblies at this point you can do so by using the same mechanism described above.
                
            
        
    


    
    
        
            
                ×
                Confirm Delete
            
            
                Are you sure you want to delete  records?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                Confirm Delete
            
            
                Are you sure you want to delete  records?
            
            
                Delete
                Cancel
            
        
    


    
    
        
            
                ×
                 Bulk Select
            
            
                
                    

                        
                            Purpose Description Prefix
                            
                                Select
                                None
                                
                            
                        

                        
                            Location
                            
                                Select
                                
                            
                        

                        

                        
                            Recovery Period
                            
                                Select
                                
                                
                            
                        

                        
                            Asset Class
                            
                                Select
                                None
                                
                            
                        


                    
                    

                        
                            Construction Division
                            
                                Select
                                None
                                
                                    1. General Requirements 
                                
                                    2. Existing Conditions 
                                
                                    3. Concrete 
                                
                                    4. Masonry 
                                
                                    5. Metals 
                                
                                    6. Wood, Plastics, and Composites 
                                
                                    7. Thermal and Moisture Protection 
                                
                                    8. Openings 
                                
                                    9. Finishes 
                                
                                    10. Specialties 
                                
                                    11. Equipment 
                                
                                    12. Furnishings 
                                
                                    13. Special Construction 
                                
                                    14. Conveying Equipment 
                                
                                    21. Fire Suppression 
                                
                                    22. Plumbing 
                                
                                    23. Heating, Ventilating, and Air Conditioning (HVAC) 
                                
                                    26. Electrical 
                                
                                    27. Communications 
                                
                                    28. Electronic Safety and Security 
                                
                                    31. Earthwork 
                                
                                    32. Exterior Improvements 
                                
                                    33. Utilities 
                                
                                    34. Transportation 
                                
                                    35. Waterway and Marine Construction 
                                
                                    41. Material Processing and Handling Equipment 
                                
                                    44. Pollution and Waste Control Equipment 
                                
                                    50. Landscaping Equipent 
                                
                                    46. Water and Wastewater Equipment 
                                
                                    48. Electrical Power Generation 
                                
                            
                        

                        
                            Record Type
                            
                                Select
                                Custom Item
                                Individual Item
                                Assemblies
                                Takeoff Cost
                            
                        

                        
                            Takeoff Cost Association
                            
                                Select
                                None
                                
                            
                        

                        
                            Units
                            
                                Select
                                None
                                
                            
                        

                        
                            Cost Source
                            
                                Select
                                None
                                
                                    Contractor Cost
                                
                                    Client Cost
                                
                                    RS Means
                                
                                    Site Contractor Cost
                                
                                    Site Indirect
                                
                                    Client Cost - Expenses
                                
                            
                        

                    
                    

                        
                            Dynamic Attribute Keys
                            
                                Select
                                None
                                
                            
                        

                        
                            Modules
                            
                                Select
                                None
                                
                            
                        

                        
                            Quantity
                            
                        

                        
                            Keyword Search (Ex: &quot;Sprinkler&quot;, &quot;~System&quot;)
                            
                        
                    
                
                
                    
                        Note: The Keyword Search tool above will not search inside of Purpose Description Prefixes. To change Purpose Description Prefixes, choose one from that Dropdown Menu
                    

                    
                        
                            
                            Select Inverse
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
    
    
        
            
                ×
                 Bulk Select By Location
            
            
                
                    
                        Location
                    
                    
                        
                            Select
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    



    
    
        
            
                ×
                 Bulk Select By Recovery Period
            
            
                
                    
                        Recovery Period
                    
                    
                        
                            Select
                            
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
    
        
            
                ×
                 Bulk Select By Asset Class
            
            
                
                    
                        Asset Class
                    
                    
                        
                            Select
                            None
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
    
        
            
                ×
                 Bulk Select By Construction Division
            
            
                
                    
                        Construction Division
                    
                    
                        
                            Select
                            None
                            
                                1. General Requirements 
                            
                                2. Existing Conditions 
                            
                                3. Concrete 
                            
                                4. Masonry 
                            
                                5. Metals 
                            
                                6. Wood, Plastics, and Composites 
                            
                                7. Thermal and Moisture Protection 
                            
                                8. Openings 
                            
                                9. Finishes 
                            
                                10. Specialties 
                            
                                11. Equipment 
                            
                                12. Furnishings 
                            
                                13. Special Construction 
                            
                                14. Conveying Equipment 
                            
                                21. Fire Suppression 
                            
                                22. Plumbing 
                            
                                23. Heating, Ventilating, and Air Conditioning (HVAC) 
                            
                                26. Electrical 
                            
                                27. Communications 
                            
                                28. Electronic Safety and Security 
                            
                                31. Earthwork 
                            
                                32. Exterior Improvements 
                            
                                33. Utilities 
                            
                                34. Transportation 
                            
                                35. Waterway and Marine Construction 
                            
                                41. Material Processing and Handling Equipment 
                            
                                44. Pollution and Waste Control Equipment 
                            
                                50. Landscaping Equipent 
                            
                                46. Water and Wastewater Equipment 
                            
                                48. Electrical Power Generation 
                            
                        
                    
                
            
            
                De-Select
                Select Items
                Cancel
            
        
    


    
    
        
            
                ×
                Associate Asset Class
            
            
                
                
                    
                        
                            
                                Choose Depreciation
                            
                        
                        
                            
                            
                            
                        
                    
                
                
                
                
                    Apply
                    Cancel
                
            
        
    




        $(document).on('click', '#associate_asset_class_btn', function(e){
            e.preventDefault();
            var valid = true;
            if(!$('#select_depreciation').val()){
                valid = false;
                $('.custom_error').text('This field is required');
            }

            if(valid){
                var row_id = $('#depreciation_row_id').val();
                $.ajax({
                    type: 'POST',
                    url: '/depreciation/associate-asset-class/',
                    data: {
                        'type': 'assign_asset_class',
                        'csrfmiddlewaretoken': 'Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2',
                        'depreciation_id': $('#select_depreciation').val(),
                        'recovery_period': $('#recovery_period_id').val()
                    },
                    success: function(response){
                        if (response){
                            $('#'+row_id+' .asset_class').multiselect('clearSelection');
                            $('#'+row_id+' .asset_class').multiselect('select', response.asset_class_ids)
                            recovery_period_manager.update_all_recovery_period($('#'+row_id+' .asset_class'));
                            $('#associate_asset_class').modal('hide');
                        }
                    }
                });
            }

        });







    
    var read_only_view = false;
    
    var notification_modal = false;

    $(&quot;#view_mode_form&quot;).on(&quot;submit&quot;, function(e){
        window.scrollTo(0, 0);
    });

    
    localStorage.setItem(&quot;show_modification_message&quot;, false)
    

    $(&quot;#id_cus_additem_form #id_unit_cost, #id_cus_additem_form #id_cus_quantity&quot;).on('keyup', function(){
        if ($(&quot;#id_cus_additem_form #id_unit_cost&quot;).val()){
            unit_cost = parseFloat($(&quot;#id_cus_additem_form #id_unit_cost&quot;).val().replace('$', '').replace(/,/g, ''));
        }
        else{
            unit_cost = 0;
        }
        if($(&quot;#id_cus_additem_form #id_cus_quantity&quot;).val()){
            quantity =  parseFloat($(&quot;#id_cus_additem_form #id_cus_quantity&quot;).val().replace(/,/, ''));
        }
        else{
            quantity = 1;
        }
        total_cost = (unit_cost * quantity).toFixed(2);
        $(&quot;#id_cus_additem_form #id_total_cost&quot;).val(total_cost.toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
    })

    $(&quot;#id_cus_additem_form #id_takeoff_cost&quot;).on(&quot;change&quot;, function(){
        if ($(&quot;#id_takeoff_cost&quot;).prop(&quot;checked&quot;) == true){
            $(&quot;#id_cus_additem_form #id_backoff_cost_error_parent_div&quot;).hide();
        }
        else{
            $(&quot;#id_cus_additem_form #id_backoff_cost_error_parent_div&quot;).show();
        }
    })

    var isAssemblyTitlesLock = false;
    var filter_active = false
    var customAssembliesList = []
    
    var customAssembliesQuantity = {}
    
    var filter_data = {}
    
    var assembliesChilds = []
    var assemblies2Childs = []
    var parentList = []
    var isAdditonalControl = false
    var isFullScreen = false
    var isToggleCost = false
    var storage = window.localStorage;
    var showModal = storage.getItem('showModal');
    var showAddItemModal = storage.getItem('showAddItemModal');
    var customAssemblyGroup = storage.getItem('search_assembly_group_no');
    var TAKEOFF_COST_OPTIONS = &quot;&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost1' selected='selected'>TC #1&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost2' >TC #2&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost3' >TC #3&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost4' >TC #4&lt;/option>&quot;
    
        TAKEOFF_COST_OPTIONS += &quot;&lt;option value='TakeoffCost5' >TC #5&lt;/option>&quot;
    
    var constructionNumber = storage.getItem('constructionNumber');
    var assemblyNumber = storage.getItem('assemblyNumber');
    var toggleCost = storage.getItem('toggleCost');
    if (toggleCost == null){
        toggleCost = false;
    }
    var view = storage.getItem('view');

    function clone_property_confirm(id){
        // $(&quot;#id_delete_confirm&quot;).attr(&quot;onclick&quot;, &quot;delete_property(&quot; + id + &quot;)&quot;);
        $('#id_clone_existing_property').val(id)
        $(&quot;#id_clone_property_modal&quot;).modal(&quot;show&quot;);
    }

    // function reset filtered items
    function resetFilteredItems(){
        event.preventDefault();
        $('.filter_item_select').val('');
        $('.filter_item_select_inverse').prop('checked', false);
        $('#reset_filter_id').val('true')
        $('#id_processing').show();
        $('#id_item_filter_form').submit()
    }

    // queryFilteredItems
    function queryFilteredItems(){
        event.preventDefault();

        var purpose_description = $('#id_filter_purpose_description').val()
        var location = $('#id_new_filter_location').val()
        var building = $('#id_new_filter_building').val()
        var asset_class = $('#id_new_filter_asset_class').val()
        var recovery_period = $('#id_new_filter_recovery_period').val()
        var quantity = $('#id_new_filter_quantity').val()
        var division = $('#id_new_filter_division').val()
        var record_type = $('#id_new_filter_type').val()
        var takeoff_cost_association = $('#id_new_filter_takeoffcost').val()
        var units = $('#id_new_filter_unit').val()
        var cost_source = $('#id_new_filter_costsource').val()
        var modules = $('#id_new_filter_module').val();
        var keyword = $('#id_new_filter_keyword').val();

        var select_inverse = $('#filter_select_inverse_check').is(':checked');

        data = {
            &quot;purpose_description&quot;: purpose_description,
            &quot;location&quot;: location,
            &quot;building&quot;: building,
            &quot;asset_class&quot;: asset_class,
            &quot;recovery_period&quot;: recovery_period,
            &quot;quantity&quot;: quantity,
            &quot;division&quot;: division,
            &quot;record_type&quot;: record_type,
            &quot;takeoff_cost_association&quot;: takeoff_cost_association,
            &quot;units&quot;: units,
            &quot;cost_source&quot;: cost_source,
            &quot;modules&quot;: modules,
            &quot;keyword&quot;: keyword,
            &quot;select_inverse&quot;: select_inverse,
        }

        $('#reset_filter_id').val('')

        $('#json_data_filter').val(JSON.stringify(data))
        $('#id_processing').show()

        $('#id_item_filter_form').submit()

    }

    function uploadCostFile(id, type){
        // $('#id_processing').show();

        $('#id_processing').show();

        var formdata = new FormData();

        formdata.append('files', $('#id_import_csv_cost').prop('files')[0]);
        formdata.append('type', type);
        formdata.append('csrfmiddlewaretoken', 'Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2');

        $.ajax({
            type: 'POST',
            url: '/project/import-cost-setup/'+id+'/',
            data: formdata,
            processData: false,
            contentType: false,
            success: function(response){

                console.log(response.cost_setup)
                for (setup in response.cost_setup){
                    form.addRow('', response.cost_setup[setup]);
                }
                $('#id_processing').hide();
                $('#importCostSetup').modal('hide');

            }
        });

    }

    function uploadFile(id, building_id){

        $('#id_processing').show();

        var formdata = new FormData();

        formdata.append('files', $('#id_import_file').prop('files')[0]);
        formdata.append('building_id', $('#import_attribute_id').val());
        formdata.append('csrfmiddlewaretoken', 'Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2');

        $.ajax({
            type: 'POST',
            url: '/project/import-dynamic-attributes/'+id+'/',
            data: formdata,
            processData: false,
            contentType: false,
            success: function(response){
                if(response.invalid_file){
                    $(&quot;#id_error_import_message&quot;).html(response.error_message);
                    $(&quot;#id_error_import_modal&quot;).modal(&quot;show&quot;);
                    return
                }

                for(var key in response.attributes){
                    if (response.attributes[key].type == &quot;number&quot;){

                        if ((response.attributes[key].value != null) &amp;&amp; (response.attributes[key].value.toString().trim() != '')){
                            $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').val(response.attributes[key].value);
                        }

                    }else if (response.attributes[key].type == &quot;option&quot;){
                        $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').val(response.attributes[key].value);

                    }else if (response.attributes[key].type == &quot;radio&quot;){
                        if (response.attributes[key].value == 'Yes'){

                            $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;][value=&quot;1&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').prop(&quot;checked&quot;, true);

                        }else{
                            $('input[value=&quot;1&quot;] .dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;][value=&quot;0&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').prop(&quot;checked&quot;, true);

                        }
                    }else{
                        $('.dynamic_attributes[data-slug=&quot;'+key+'&quot;][data-building_id=&quot;'+response.building_id+'&quot;]:not(:disabled):not([readonly])').not('.tenant_name_trigger').val(response.attributes[key].value);
                    }

                }
                if(!response.correct_data){
                    $(&quot;#id_error_import_message&quot;).html(response.error_message);
                    $(&quot;#id_error_import_modal&quot;).modal(&quot;show&quot;);
                }

                $('.dynamic_attributes[data-building_id=&quot;'+response.building_id+'&quot;]').last().trigger('change');
                $('#id_processing').hide();
                $('#importDynamicAttribute').modal('hide');
            }
        })
    }

    function show_hide_child_heirarchy() {
        ''
    }

    $(&quot;.alert&quot;).delay(5000).slideUp(300);
    var state = $('input[name=state_status]:checked').val();

    var is_modal_open = false;

    
        $('#applySqueeze').on('shown.bs.modal', function (e) {
            $(&quot;#id_squeeze_by_custom_backoff_cost_input&quot;).show();
        })
    

    $('#add_item_assembly').on('shown.bs.modal', function (e) {
        is_modal_open = true;
        $(&quot;#id_item&quot;).focus();
    })

    $('#add_item_assembly').on('shown.bs.modal', function (e) {
        if(!showAddItemModal){
            $(&quot;input[id='hidden_value_for_check']&quot;).val('')
            trigger_events($('#id_additem_form'));
            calculate_global_pricing();
            trigger_events($('#id_cus_additem_form'));
        }
    })

    $(&quot;#deleteZeroQuatityModal&quot;).on('show.bs.modal', function(e){
        deSelectFilteredItems();
        selectedIndex = [];

        var quantity = &quot;0&quot;;

        var target = $('[data-id]');
        target.each(function() {
            var item_quantity = $(this).siblings(&quot;.quantity&quot;).text();
            var isSelected = true;

            if(quantity &amp;&amp; parseFloat(quantity) != parseFloat(item_quantity)){
                isSelected = false;
            }

            if(isSelected){
                $(this).addClass('selected');
                selectedIndex.push($(this).attr('data-id'));
            }
        });
        $(&quot;input[name='id_lists']&quot; ).val(selectedIndex);
        var records = selectedIndex.length;

        if(!records){
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#nozeroquantity&quot;).modal(&quot;show&quot;);
        }
        else{
            $('.id_records_count').html('&lt;b>'+records+'&lt;/b>');
        }
    })

    $(&quot;#deleteZeroQuatityModal&quot;).on('hidden.bs.modal', function(e){
        deSelectFilteredItems();
    })

    $('#summarysection').on('hidden.bs.modal', function () {
        is_modal_open = false;
    })

    $('#add_item_assembly').on('hidden.bs.modal', function () {
        is_modal_open = false;
        storage.setItem('showModal', '');
        storage.setItem('showAddItemModal', '');
        storage.setItem('constructionNumber', '');
        storage.setItem('assemblyNumber', '');
        const_div = '';
        assmb_grp = '';

        if($(&quot;#normal&quot;).hasClass('active')){
            save_input($('#id_additem_form'));
        }else{
            save_input($('#id_cus_additem_form'));
        }

        setTimeout(function(){
            window.location.reload();
        });
    })

    $(&quot;#id_custom_assembly_form&quot;).submit(function(e){
        e.preventDefault();
        return false;
    });

    function bindModalEvent() {
        $('.modal').each(function(){
            if($(this).attr('id') != 'add_item_assembly'){
                $(this).on('hidden.bs.modal', function (e) {
                    is_modal_open = false;
                });
            }
        });
    }

    bindModalEvent();

    var INACTIVE_TIME = new Date().getTime();
    $(document.body).bind(&quot;click&quot;, function (e) {
        INACTIVE_TIME = new Date().getTime();
    });

    $(document.body).bind(&quot;keypress&quot;, function () {
        INACTIVE_TIME = new Date().getTime();
    });

    $(&quot;#id_item_group_no&quot;).change(function () {
        var value = this.value;
        if (value == &quot;&quot; ){
            $(&quot;#id_item_sub_group_no&quot;).remove();
            $(&quot;#id_disabledgroup&quot;).show();
            $(&quot;#append_item_sub_group_no&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no&quot;>\
                        &lt;option value=&quot;&quot;>Select&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_disabledgroup&quot;).hide();
                    $(&quot;#append_item_sub_group_no&quot;).show();
                    $(&quot;#append_item_sub_group_no&quot;).html(select);
                    $(document).trigger(&quot;item_subgroup&quot;);
                }
            });
        }
    });

    $(&quot;#id_squeezetype&quot;).change(function(){
        val = $(this).val();
        if(val == &quot;squeeze_by_cons_div&quot;){
            $(&quot;#id_squeeze_cons_div_no_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_cons_div_no_input&quot;).hide();
        }

        if(val == &quot;squeeze_by_uniformat&quot;){
            $(&quot;#id_squeeze_uniformat_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_uniformat_input&quot;).hide();
        }

        if(val == &quot;squeeze_by_recovery_period&quot;){
            $(&quot;#id_squeeze_recovery_period_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_recovery_period_input&quot;).hide();
        }
        if(val == &quot;squeeze_by_location&quot;){
            $(&quot;#id_squeeze_location_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_location_input&quot;).hide();
        }
        if(val == &quot;squeeze_entire_takeoffsheet&quot;){
            $(&quot;#id_squeeze_entire_sheet_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_entire_sheet_input&quot;).hide();
        }if(val == &quot;squeeze_by_custom_backoff_cost&quot;){
            $(&quot;#id_squeeze_by_custom_backoff_cost_input&quot;).show();
        }else{
            $(&quot;#id_squeeze_by_custom_backoff_cost_input&quot;).hide();
        }
    });

    $('input[type=radio][name=search_option]').change(function() {
        search_option = $(&quot;input[name='&quot; + $(this).attr('name') + &quot;']:checked&quot;).val();
        $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;);
        getCuratedItems(async=true);
    });

    $('input[type=radio][name=search_option_1]').change(function() {
        search_option = $(&quot;input[name='&quot; + $(this).attr('name') + &quot;']:checked&quot;).val();
        $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;);
        getCuratedItems2(async=true);
    });

    search_option = &quot;curated&quot;
    state_status = &quot;default&quot;
    const_div = &quot;&quot;
    assmb_grp = &quot;&quot;
    sub_group = &quot;&quot;
    assemb_sub_group = &quot;&quot;

    $('#id_construction_division').change(function() {
        const_div = $(this).val();
        $(&quot;#id_item_sub_group_search_div&quot;).hide();
        sub_group = &quot;&quot;
        getCuratedItems(async=true);
        var value = $(this).find(':selected').attr('data-division-value');
        if (value == undefined ){
            $(&quot;#id_item_sub_group_search_div&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no_items&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_item_sub_group_search_div&quot;).show();
                    $(&quot;#id_item_sub_group_search_div&quot;).html(select);
                    $('#id_item_sub_group_no_items').change(function() {
                        sub_group = $(this).val();
                        getCuratedItems(async=true);
                    });
                    $(document).trigger('subconstruction_loaded')
                }else{
                    $(&quot;#id_item_sub_group_search_div&quot;).hide();
                }
                }
            });
        }
    });


    $('#id_search_assembly_group_no').change(function() {
        assmb_grp = $(this).val();
        getCuratedItems(async=true);

        $(&quot;#id_assembly_sub_group_search_div&quot;).hide();

        assemb_sub_group = &quot;&quot;

        var value = $(this).val();
        if (value == undefined || value == ''){
            $(&quot;#id_assembly_sub_group_search_div&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-assembly-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;assembly_sub_group_no&quot; id=&quot;id_assembly_sub_group_no_items&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                        for(var i=0; i&lt;response.length; i++){
                            var option = &quot;&lt;option value=&quot;+response[i].id+&quot;>&quot;+ response[i].id + &quot;. &quot; + response[i].title + &quot;&lt;/option>&quot;;
                            $(select).append(option)
                        }
                        $(&quot;#id_assembly_sub_group_search_div&quot;).show();
                        $(&quot;#id_assembly_sub_group_search_div&quot;).html(select);

                        $('#id_assembly_sub_group_no_items').change(function() {
                            assemb_sub_group = $(this).val();
                            getCuratedItems();
                        });
                    }else{
                        $(&quot;#id_assembly_sub_group_search_div&quot;).hide();
                    }
                }
            });
        }
    });

    $('#id_construction_division_1').change(function() {
        $(&quot;#id_search_items&quot;).val(&quot;&quot;);
        const_div = $(this).val();
        $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
        sub_group = &quot;&quot;
        getCuratedItems2();
        var value = $(this).find(':selected').attr('data-division-value');
        if (value == undefined ){
            $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no_items_1&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_item_sub_group_search_div_1&quot;).show();
                    $(&quot;#id_item_sub_group_search_div_1&quot;).html(select);
                    $('#id_item_sub_group_no_items_1').change(function() {
                        sub_group = $(this).val();
                        getCuratedItems2();
                    });
                    $(document).trigger('subconstruction_loaded1')
                }
                else{
                    $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
                }
                }
            });
        }
    });

    $('#id_construction_division_2').change(function() {
        const_div = $(this).val();
        $(&quot;#id_item_sub_group_search_div_1&quot;).hide();
        sub_group = &quot;&quot;

        var value = $(this).find(':selected').attr('data-division-value');
        if (value == undefined ){
            $(&quot;#id_item_sub_group_search_div_2&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-item-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;item_sub_group_no&quot; id=&quot;id_item_sub_group_no_items_2&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    for(var i=0; i&lt;response.length; i++){
                        var option = &quot;&lt;option value=&quot;+response[i].item_no+&quot;>&quot;+response[i].count +&quot;. &quot;+ response[i].item_desc + &quot; &quot; + &quot;(&quot;+response[i].item_no+&quot;)&quot; + &quot;&lt;/option>&quot;;
                        $(select).append(option)
                    }
                    $(&quot;#id_item_sub_group_search_div_2&quot;).show();
                    $(&quot;#id_item_sub_group_search_div_2&quot;).html(select);
                }
            });
        }
    });

    $('#id_search_assembly_group_no_1').change(function() {
        $(&quot;#id_search_items&quot;).val(&quot;&quot;);
        assmb_grp = $(this).val();
        getCuratedItems2();

        $(&quot;#id_assembly_sub_group_search_div_1&quot;).hide();

        assemb_sub_group = &quot;&quot;

        var value = $(this).val();
        if (value == undefined || value == ''){
            $(&quot;#id_assembly_sub_group_search_div_1&quot;).hide();
        }else{
            $.ajax({
                url: &quot;/project/get-assembly-subgroup/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'value': value,
                },
                success: function(response){
                    response = JSON.parse(response);
                    var select = '&lt;select class=&quot;form-control input-sm&quot; name=&quot;assembly_sub_group_no&quot; id=&quot;id_assembly_sub_group_no_items_1&quot;>\
                        &lt;option value=&quot;&quot;>Select SubGroup&lt;/option>\
                    &lt;/select>';
                    select = $(select);
                    if(response.length > 0){
                        for(var i=0; i&lt;response.length; i++){
                            var option = &quot;&lt;option value=&quot;+response[i].id+&quot;>&quot;+ response[i].id + &quot;. &quot; + response[i].title + &quot;&lt;/option>&quot;;
                            $(select).append(option)
                        }
                        $(&quot;#id_assembly_sub_group_search_div_1&quot;).show();
                        $(&quot;#id_assembly_sub_group_search_div_1&quot;).html(select);

                        $('#id_assembly_sub_group_no_items_1').change(function() {
                            assemb_sub_group = $(this).val();
                            getCuratedItems2();
                        });
                    }else{
                        $(&quot;#id_assembly_sub_group_search_div_1&quot;).hide();
                    }
                }
            });
        }
    });

    

    

    window.onbeforeunload = function(event) {
        window.localStorage.setItem('Xoffset', window.pageXOffset)
        window.localStorage.setItem('Yoffset', window.pageYOffset)
    };

    $(document).ready(function () {
        // $('#id_filter_purpose_description').val(filter_data.purpose_description)
        // $('#id_new_filter_location').val(filter_data.location)
        // $('#id_new_filter_building').val(filter_data.building)
        // $('#id_new_filter_asset_class').val(filter_data.asset_class)
        // $('#id_new_filter_recovery_period').val(filter_data.recovery_period)
        // $('#id_new_filter_quantity').val(filter_data.quantity)
        // $('#id_new_filter_division').val(filter_data.division)
        // $('#id_new_filter_type').val(filter_data.record_type)
        // $('#id_new_filter_takeoffcost').val(filter_data.takeoff_cost_association)
        // $('#id_new_filter_unit').val(filter_data.units)
        // $('#id_new_filter_costsource').val(filter_data.cost_source)
        // $('#id_new_filter_module').val(filter_data.module);
        // $('#id_new_filter_keyword').val(filter_data.keyword);

        if(filter_data){
            if(filter_data.purpose_description || filter_data.location || filter_data.building || filter_data.asset_class || filter_data.recovery_period || filter_data.quantity || filter_data.division || filter_data.record_type || filter_data.takeoff_cost_association || filter_data.units || filter_data.cost_source || filter_data.module || filter_data.keyword){
                filter_active = true
            }else{
                filter_active = false
            }
        }

        if (filter_active){
            $('#filter-circle').show();
            $('#filter-color-change').css('color', 'green');
            $('#filter-color-change').css('background', 'red');
            $('#filter-change-name').html('Active Filter');
        }

        $('td.quantity, #id_update_quantity').on('keydown', function(e){
            var code = e.keyCode

            if(code == 189){
                return false;
            }
        });
        // $('.building_types').multiselect({
        //     buttonWidth: '100%',
        //     nonSelectedText: 'Select',
        //     enableFiltering: false,
        //     maxHeight: 300,
        // });

        var newItemId = window.localStorage.getItem('NewItemId');

        if (newItemId){
            window.localStorage.setItem('NewItemId', '');
            //~ $(&quot;html, body&quot;).animate({
                //~ scrollTop: $('#item-id-'+newItemId).offset().top - $(&quot;body > nav&quot;).outerHeight() - $(&quot;#content > table:nth-child(2) > thead&quot;).height(),
            //~ }, 500);
            window.setTimeout(function() {
                window.scrollTo(window.localStorage.getItem('Xoffset'), $('#item-id-'+newItemId).offset().top - $(&quot;body > nav&quot;).outerHeight() - $(&quot;#content > table:nth-child(2) > thead&quot;).height() - 40)
            }, 500)
        }else{
            
                //window.setTimeout(function() {
                //    window.scrollTo(window.localStorage.getItem('Xoffset'), window.localStorage.getItem('Yoffset'))
                //}, 600)
            
        }

        var tmp = $('input#id_tie_to_no_apply_squeeze').val();

        var tmp3 = Number(tmp).toLocaleString(&quot;en-US&quot;, {style:&quot;currency&quot;, currency:&quot;USD&quot;, minimumFractionDigits: 2});

        if (tmp3 == 'NaN' | tmp3 == '$NaN' | tmp3 == '') {
            $('input#id_tie_to_no_apply_squeeze').val(tmp)
        } else {
            $('input#id_tie_to_no_apply_squeeze').val(tmp3);
        }

        $('input#id_tie_to_no_apply_squeeze').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }
        });

        $('#id_tie_to_no_apply_squeeze').bind('paste', function () {
            var self = this;
            setTimeout(function () {
                if (!/\d*(\.\d{1,2})+$/.test($(self).val())) $(self).val('');
            }, 0);
        });

        $('#id_custom_assembly_name').keypress(function(event) {
            $(&quot;#id_name_error&quot;).hide();
        });

        $('input[type=radio][name=level]').change(function() {
            $(&quot;#id_level_error&quot;).hide();
        });

        $('input#id_tie_to_no_apply_squeeze').keyup(function(e) {
            if ($(this).val().replace('$','').replace(',','').length &lt;= 13){
                $('#apply_squeeze_button_action').prop('disabled', false);
                $('.apply_squeeze_error').html('')
                if ($(this).val() == &quot;0.0&quot;){
                    $(this).val() = &quot;&quot;
                }else{
                    var $this = $(this);
                    var num = $this.val().replace(/,/g, '');
                    $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
                }
            }else{
                $('.apply_squeeze_error').html('Invalid Input')
                $('#apply_squeeze_button_action').prop('disabled', true);

            }
        });

        var ParentId = window.localStorage.getItem('ParentId')
        if (ParentId) {
            $(&quot;.&quot; + ParentId).show()
            window.localStorage.removeItem('ParentId')
        }

        $(&quot;#id_toggle_nav_action&quot;).click(function(){
            $(&quot;#id_nav_action&quot;).slideToggle();
            isAdditonalControl = !isAdditonalControl;
            UpdateTableHeader();
        });

        $( function() {
            $(&quot;#id_is_curated&quot;).click(function(){
                var curated_item_id = $(&quot;#item_hidden_id&quot;).val();
                var curated_item_status = $(&quot;#id_is_curated&quot;).is(':checked');
                $.ajax({
                    url: &quot;/project/change-item-curated/&quot;,
                    type: &quot;GET&quot;,
                    data: {
                        'curated_item_id': curated_item_id,
                        'curated_item_status': curated_item_status
                    },
                    success: function(response){
                        if (response.status) {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated&quot;).prop('checked', true);
                            } else {
                                $(&quot;#id_is_curated&quot;).prop('checked', false);
                            }
                        } else {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated&quot;).prop('checked', false);
                            } else {
                                $(&quot;#id_is_curated&quot;).prop('checked', true);
                            }
                        }
                    }
                })
            })
        });

        $( function() {
            $(&quot;#id_is_curated_1&quot;).click(function(){
                var curated_item_id = $(&quot;#item_hidden_id_1&quot;).val();
                var curated_item_status = $(&quot;#id_is_curated_1&quot;).is(':checked');
                $.ajax({
                    url: &quot;/project/change-item-curated/&quot;,
                    type: &quot;GET&quot;,
                    data: {
                        'curated_item_id': curated_item_id,
                        'curated_item_status': curated_item_status
                    },
                    success: function(response){
                        if (response.status) {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', true);
                            } else {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', false);
                            }
                        } else {
                            if (curated_item_status) {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', false);
                            } else {
                                $(&quot;#id_is_curated_1&quot;).prop('checked', true);
                            }
                        }
                    }
                })
            })
        });

        $(function() {
            $('.multiselect-ui').multiselect({
                includeSelectAllOption: true
            });
        });

        add_border();

        $( function() {
            $('.two-digits').keyup(function(){
                if($(this).val().indexOf('.') != -1){
                    if($(this).val().split(&quot;.&quot;)[1].length > 2) {
                        if( isNaN( parseFloat( this.value ) ) ) return;
                        this.value = parseFloat(this.value).toFixed(2);
                    }
                 }
                 return this;
            });
        });

        

        $(document).keyup(function(e) {
            if (e.keyCode === 27){
                deSelectFilteredItems();
            }
        });

        function nonSelect(){
            $('.modal').modal('hide');
            is_modal_open = true;
            $(&quot;#nonselected&quot;).modal('show');
        }

        
            property_id = &quot;&quot;
        

        $(&quot;#id_cus_item_property&quot;).val(property_id);
        $(&quot;#id_quantity_property&quot;).val(property_id);
        $(&quot;#id_con_property&quot;).val(property_id);
        $(&quot;#id_loc_property&quot;).val(property_id);

        $('#id_quantity, #id_cus_quantity, #item_details #id_edit_quantity, #id_aitem_edit_quantity, #id_unit_cost').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }
        });

        $('#update_purpose_description').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_con&quot;).focus();
        });

        $('#update_filtered_purpose_description').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_filter_purpose&quot;).focus();
        });

        $('#update_asset_year').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_asset&quot;).focus();
        });

        $('#update_recovery_period').on('shown.bs.modal', function() {
            is_modal_open = true
            var records = selectedIndex.length;

            $('#id_records_count_asset').html('&lt;b>'+records+'&lt;/b>');

            $(&quot;#id_edit_bulk_class&quot;).focus();
        });

        $('#update_cost_source').on('shown.bs.modal', function() {
            is_modal_open = true
            var records = selectedIndex.length;

            $('#id_records_count_cost_source').html('&lt;b>'+records+'&lt;/b>');

        });

        $('#update_location').on('shown.bs.modal', function() {
            is_modal_open = true
            $(&quot;#id_edit_bulk_loc&quot;).focus();
        });

        $('#id_quantity').keyup(function(e) {
            var $this = $(this);
            var num = $this.val().replace(/,/g, '');
            $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
        });

        $('#id_total_cost').keyup(function(e) {
            var $this = $(this);
            var num = $this.val().replace(/,/g, '');
            $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
        });

        $('#id_unit_cost').keyup(function(e) {
            var $this = $(this);
            var num = $this.val().replace(/,/g, '');
            $this.val(num.replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;));
        });

        $('#id_total_cost').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }else{
                var velement = event.target || event.srcElement
                var fstpart_val = velement.value;
                var parts = velement.value.split('.');
                if (parts.length == 2 &amp;&amp; parts[1].length >= 2) event.preventDefault();
            }
        });

        $('#id_unit_cost').keypress(function(event) {
            if ((event.which != 46 || $(this).val().indexOf('.') != -1) &amp;&amp; (event.which &lt; 48 || event.which > 57)) {
                event.preventDefault();
            }else{
                var velement = event.target || event.srcElement
                var fstpart_val = velement.value;
                var parts = velement.value.split('.');
                if (parts.length == 2 &amp;&amp; parts[1].length >= 2) event.preventDefault();
            }
        });

        $('[data-toggle=&quot;tooltip&quot;]').tooltip();

        $('#id_location').change(function() {
            if (this.value == &quot;Others&quot;){
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    $(&quot;#id_building_error_parent_div&quot;).show();
                }
                $(&quot;#id_extra_location_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_location_error_parent_div&quot;).hide();
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                        $(&quot;#id_building_error_parent_div&quot;).show();
                    }else{
                        $(&quot;#id_building_error_parent_div&quot;).hide();
                    }
                }
            }
        });

        $('#id_map_location').change(function(){
            console.log($(this).val())
            if ($(this).val() == &quot;Others&quot;){
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    $(&quot;#id_map_location_building_div&quot;).show();
                }
                $(&quot;#id_map_add_extra_location&quot;).show();
            }else{
                $(&quot;#id_map_add_extra_location&quot;).hide();
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                        $(&quot;#id_map_location_building_div&quot;).show();
                    }else{
                        $(&quot;#id_map_location_building_div&quot;).hide();
                    }
                }
            }
        });

        $('#id_location_algo').change(function() {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_location_algo_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_location_algo_error_parent_div&quot;).hide();
            }
        });

        $('#id_cus_location').change(function() {
            if (this.value == &quot;Others&quot;){
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    $(&quot;#id_cus_building_error_parent_div&quot;).show();
                }
                $(&quot;#id_cus_extra_location_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_cus_extra_location_error_parent_div&quot;).hide();
                if($(&quot;#id_property_building_count&quot;).val() > 1){
                    if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                        $(&quot;#id_cus_building_error_parent_div&quot;).show();
                    }else{
                        $(&quot;#id_cus_building_error_parent_div&quot;).hide();
                    }
                }
            }
        });

        $('#id_purpose_description').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).hide();
            }
        });

        $('#id_purpose_description_algo').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_purpose_description_algo_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_purpose_description_algo_error_parent_div&quot;).hide();
            }
        });

        $('#id_purpose_description_custom').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_purpose_description_error_parent_div&quot;).hide();
            }
        });

        $('#id_cost_source').on('change', function (e) {
            if (this.value == &quot;Others&quot;){
                $(&quot;#id_extra_cost_source_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_cost_source_error_parent_div&quot;).hide();
            }
        });

        $('#id_cus_unit').on('change', function (e) {
            if (this.value == &quot;Othrs&quot;){
                $(&quot;#id_extra_cus_unit_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_cus_unit_error_parent_div&quot;).hide();
            }
        });

        $('#id_cus_unit_assembly').on('change', function (e) {
            if (this.value == &quot;Othrs&quot;){
                $(&quot;#id_extra_cus_unit_assembly_error_parent_div&quot;).show();
            }else{
                $(&quot;#id_extra_cus_unit_assembly_error_parent_div&quot;).hide();
            }
        });

        $('#id_recovery_period').change(function() {
            if (this.value == &quot;GDS_sc&quot;){
                $('#id_location').prop('disabled', true);
                $('#id_location').val('None')

            }else{
                $('#id_location').prop('disabled', false);
            }
        });

        $('#id_cus_recovery_period').change(function() {
            if (this.value == &quot;GDS_sc&quot;){
                $('#id_cus_location').prop('disabled', true);
                $('#id_cus_location').val('None')

            }else{
                $('#id_cus_location').prop('disabled', false);
            }
        });

        $(window).scroll(function () {
            if ($(this).scrollTop() > 50) {
                $('#back-to-top').fadeIn();
            } else {
                $('#back-to-top').fadeOut();
            }
        });

        $('#back-to-top').click(function () {
            $('#back-to-top').tooltip('hide');
            $('body,html').animate({
                scrollTop: 0
            }, 800);
            return false;
        });

        $('#back-to-top').tooltip('show');

        $('#changepassword').on('shown.bs.modal', function() {
            $('.cs-error').hide();
            $('.form-group').removeClass('has-error');
        });
    });

    $(&quot;#other_information_btn&quot;).click(function(){
        var selectedEffect = &quot;fade&quot;;
        $(&quot;.other_information&quot;).toggle( selectedEffect, 500, callback );
        $(this).html($(this).html() == 'Show Others Fields' ? 'Hide Others Fields' : 'Show Others Fields');
    });

    function callback() {
        setTimeout(function() {
            $( &quot;#effect&quot; ).removeAttr( &quot;style&quot; ).hide().fadeIn();
        }, 400 );
    };

    $(document).bind('keypress', function(e) {
        var code = e.shiftKey &amp;&amp; e.which;
        if (code === 69){
            if (!is_modal_open){
                ''
                    show_hide_child();
                ''
            }
        }
    });

    $.ajaxTransport(&quot;+binary&quot;, function(options, originalOptions, jqXHR){
        // check for conditions and support for blob / arraybuffer response type
        if (window.FormData &amp;&amp; ((options.dataType &amp;&amp; (options.dataType == 'binary'))
            || (options.data
            &amp;&amp; ((window.ArrayBuffer &amp;&amp; options.data instanceof ArrayBuffer)
            || (window.Blob &amp;&amp; options.data instanceof Blob))))
        )
        {
            return {
                // create new XMLHttpRequest
                send: function(headers, callback){
            // setup all variables
                    var xhr = new XMLHttpRequest(),
            url = options.url,
            type = options.type,
            async = options.async || true,
            // blob or arraybuffer. Default is blob
            dataType = options.responseType || &quot;blob&quot;,
            data = options.data || null,
            username = options.username || null,
            password = options.password || null;
                    xhr.addEventListener('load', function(){
                var data = {};
                data[options.dataType] = xhr.response;
                // make callback and send data
                callback(xhr.status
                        , xhr.statusText
                        , data
                        , xhr.getAllResponseHeaders());
                    });

                    xhr.open(type, url, async, username, password);

            // setup custom headers
            for (var i in headers ) {
                xhr.setRequestHeader(i, headers[i] );
            }

                    xhr.responseType = dataType;
                    xhr.send(data);
                },
                abort: function(){
                    jqXHR.abort();
                }
            };
        }
    });

    function revert_items_cost(){
        revert_ajax_request(&quot;/project/revert-cost-factors/&quot;, function(response){
            location.reload();
        });
    }

    function revert_items_cost_and_reopen(){
        revert_ajax_request(&quot;/project/revert-cost-factors/&quot;, function(response){
            window.location.href = window.location.href.split(&quot;?&quot;)[0] + '?page=';
        })
    }

    function bulk_ajax_request($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;GET&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
            },
            success: $f
        });
    }

    function bulk_ajax_delete_request($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;POST&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
                'csrfmiddlewaretoken': &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;
            },
            success: $f
        });
    }

    function bulk_ajax_delete_request_filter($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;POST&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
                'csrfmiddlewaretoken': &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;,
                &quot;delete_filtered_zeroes&quot;: true
            },
            success: $f
        });
    }

    function revert_ajax_request($url, $f){
        localStorage.setItem(&quot;show_modification_message&quot;, true);
        $(&quot;#id_processing&quot;).show();
        $.ajax({
            url: $url,
            type: &quot;POST&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
                'csrfmiddlewaretoken': &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;
            },
            success: $f
        });
    }

    function update_totalcost_from_db_and_reopen(){
        bulk_ajax_request(&quot;/project/update-totalcost-db/&quot;, function(response){
            window.location.href = window.location.href.split(&quot;?&quot;)[0] + '?page=';
        });
    }

    function duplicate_items_and_reopen(){
        bulk_ajax_request(&quot;/project/duplicate-items/&quot;, function(response){
            window.location.href = window.location.href.split(&quot;?&quot;)[0] + '?page=';
        });
    }

    function downloadFile(url, headers, filename) {
        $(&quot;#id_processing&quot;).show();
        return $.ajax({
            url:url,
            method: &quot;POST&quot;,
            // dataType:&quot;binary&quot;,
            data: {
                'current': window.location.href,
                'csrfmiddlewaretoken': &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;
            },
            // processData: false,
            // headers:headers
            success: function(){
                // $(&quot;#id_processing&quot;).hide();
                $(&quot;#show-report-success&quot;).show();
                setTimeout(function(){$(&quot;#show-report-success&quot;).hide()}, 2000)
            }
        })
    }

    function exportsheet(property_pk) {
        // $(&quot;#id_processing&quot;).show();
        if(UpdateQuantityRequestInProgress.length &lt;= 0){
            var url = &quot;/project/export-excel-sheet/&quot;+ property_pk + '/';
            var sheet_name = ''
            sheet_name = sheet_name.replace(/[-!#$%^&amp;*()_+|~=`{}\[\]:&quot;;'&lt;>?,.\/]/g, '')
                                    .replace(/\s/g, '_')
            var filename = &quot;TakeoffSheet&quot; + sheet_name + &quot;.zip&quot;

            downloadFile(url, {&quot;X-Requested-With&quot;:&quot;XMLHttpRequest&quot;}, filename);
        }
    }

    
    var selectedIndex = [];
    

    function initializeSelectedIndex() {
        if (selectedIndex){
            for(index of selectedIndex){
                $(&quot;[select-data-id='&quot;+index+&quot;']&quot;).addClass(&quot;selected&quot;);
                $(&quot;.pre-dis&quot;).prop(&quot;disabled&quot;, false);
                // $(&quot;#bulk_tool_select&quot;).modal(&quot;show&quot;);
            }
            $(&quot;input[name='id_lists']&quot;).val(selectedIndex);
        }
    };

    $(&quot;#id_delete_item&quot;).on('click', function(e){
        var records = selectedIndex.length;
        $('.id_records_count').html('&lt;b>'+records+'&lt;/b>');
        $('#deleteConfirmModal').modal(&quot;show&quot;);
    });

    $(&quot;#update_totalcost_db&quot;).on('shown.bs.modal', function(e){
        var records = selectedIndex.length;

        $('.id_records_count').html('&lt;b>'+records+'&lt;/b>');
    });

    $('#warnings_dialogue_link').on('click', function(){
        var property_type = ''
        if(property_type == &quot;PPA&quot;){
            $('#warningdialogueNEWtoPPA').modal(&quot;show&quot;);
        } else if(property_type == &quot;NEW&quot;) {
            $('#warningdialoguePPAtoNEW').modal(&quot;show&quot;);
        }
    })

    $('#id_edit_bulk_con').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            $(&quot;#id_edit_bulk_extra_purpose_description&quot;).show();
        }else{
            $(&quot;#id_edit_bulk_extra_purpose_description&quot;).hide();
        }
    });

    $('#id_edit_bulk_filter_purpose').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            $(&quot;#id_filter_bulk_extra_purpose_description&quot;).show();
        }else{
            $(&quot;#id_filter_bulk_extra_purpose_description&quot;).hide();
        }
    });

    $('#id_edit_bulk_loc').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                $(&quot;#id_edit_bulk_building&quot;).show();
            }
            $(&quot;#id_edit_bulk_extra_location&quot;).show();
        }else{
            $(&quot;#id_edit_bulk_extra_location&quot;).hide();
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                    $(&quot;#id_edit_bulk_building&quot;).show();
                }else{
                    $(&quot;#id_edit_bulk_building&quot;).hide();
                }
            }
        }
    });

    $('#id_edit_bulk_loc_filter').on('change', function (e) {
        if ($(this).val() == &quot;Others&quot;){
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                $(&quot;#id_edit_bulk_building_filter&quot;).show();
            }
            $(&quot;#id_edit_bulk_extra_location_filter&quot;).show();
        }else{
            $(&quot;#id_edit_bulk_extra_location_filter&quot;).hide();
            if($(&quot;#id_property_building_count&quot;).val() > 1){
                if(!$(this).find(&quot;option:selected&quot;).data(&quot;site&quot;)){
                    $(&quot;#id_edit_bulk_building_filter&quot;).show();
                }else{
                    $(&quot;#id_edit_bulk_building_filter&quot;).hide();
                }
            }
        }
    });

    $(&quot;#id_update_qantity&quot;).on('click', function(e){
        $.ajax({
            url: &quot;/project/update-all-quantiity/&quot;,
            type: &quot;GET&quot;,
            data: {
                'selected_ids' : selectedIndex.join('_'),
            },
            success: function(response){
                location.reload();
            }
        });
    });

    $(document).on('dblclick', '[data-id]:not([assmb-item-id])', function(e){
        e.preventDefault();
        var self = $(this),
        item_id = self.attr('data-id');
        // $('#item_details_modal').remove();
        if (!read_only_view_enabled){
            $.ajax({
                url: &quot;/project/record-details/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'item_id' : item_id,
                    'property_id': property_id,
                    'view_mode': $(&quot;[name='view_mode']:checked&quot;).val()
                },
                success: function(response){
                    $(&quot;#item_details_modal&quot;).html(response.data);
                    is_modal_open = true
                    bindModalEvent();
                    $('#item_details').modal('show');
                    $('#id_edit_con').on('change', function (e) {
                        if (this.value == &quot;Others&quot;){
                            $(&quot;#id_edit_extra_purpose_description_error_parent_div&quot;).show();
                        }else{
                            $(&quot;#id_edit_extra_purpose_description_error_parent_div&quot;).hide();
                        }
                    });
                    $('#id_edit_loc').on('change', function (e) {
                        if (this.value == &quot;Others&quot;){
                            $(&quot;#id_edit_extra_location_error_parent_div&quot;).show();
                        }else{
                            $(&quot;#id_edit_extra_location_error_parent_div&quot;).hide();
                        }
                    });
                }
            });
        }else{
            e.stopPropagation();
        }
    });

    $(document).on('click', '[select-data-id]', function(e){

        e.preventDefault();
        var self = $(this),
        item_id = self.attr('select-data-id');
        // if(!read_only_view_enabled){
        if(e.shiftKey || e.ctrlKey) {
            e.stopPropagation();
            if(self.hasClass('selected')) {
                self.removeClass('selected');
                selectedIndex.splice(selectedIndex.indexOf(item_id), 1);
                $(&quot;input[name='id_lists']&quot; ).val(selectedIndex);

            } else {
                self.addClass('selected');
                selectedIndex.push(item_id);
                $(&quot;input[name='id_lists']&quot; ).val(selectedIndex);

            }
            if(selectedIndex.length === 0) {
                $(&quot;.pre-dis&quot;).attr(&quot;disabled&quot;, &quot;true&quot;);
            } else {
                $(&quot;.pre-dis&quot;).removeAttr(&quot;disabled&quot;);
            }
        }
        // }else{
        //     e.stopPropagation();
        // }

    });

    $(document).on('click', '[record-id]', function(e){
        if(read_only_view_enabled){
            e.preventDefault();
            e.stopPropagation();
        }else{
            $(this).prop('contenteditable', true);
        }

        //$('td[contenteditable]').focus(function(e) {
        //    document.execCommand('insertHTML', false);
        //    document.execCommand('selectAll', false);
        //    return false;
        //});

        //$('td[contenteditable]').keypress(function(e){ return e.which != 13; });
    });

    $(document).on('focus', '[record-id]', function(e){
        // document.execCommand('selectAll', false);
        $(this).selectText();
    });

    $(document).on('keypress', '[record-id]', function(e){ return e.which != 13; });

    $(document).on('dblclick', '[assmb-item-id]', function(e){
        var self = $(this),
        item_id = self.attr('assmb-item-id');
        e.preventDefault();

        if(!read_only_view_enabled){
            $.ajax({
                url: &quot;/project/assemb-item-details/&quot;,
                type: &quot;GET&quot;,
                data: {
                    'item_id' : item_id,
                    'view_mode': $(&quot;[name='view_mode']:checked&quot;).val()
                },
                success: function(response){
                    $(&quot;#item_details_modal&quot;).html(response.data);
                    is_modal_open = true;
                    bindModalEvent();
                    $('#item_details').modal('show');
                }
            });
        }else{
            e.stopPropagation();
        }
        // $('#item_details_modal').remove();
    });

    var selectAllItems = true;
    $('#id_select_item').on('click',async function(e) {
        if(is_all_items_loaded){
            e.preventDefault();
            select_all_items();
            $(&quot;#id_processing&quot;).hide();
        }
        else{
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#id_processing&quot;).show();
            res = await new Promise(function(resolve, reject) {
                setInterval(function(){ if(is_all_items_loaded){resolve(&quot;Done!&quot;)}; }, 3000);
            });
            $(this).click();
        }
    });

    $('#id_select_inverse_item').on('click', async function(e){
        if(is_all_items_loaded){
            e.preventDefault();
            select_all_inverse_items();
            $(&quot;#id_processing&quot;).hide();
        }else{
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#id_processing&quot;).show();
            res = await new Promise(function(resolve, reject) {
                setInterval(function(){ if(is_all_items_loaded){resolve(&quot;Done!&quot;)}; }, 3000);
            });
            $(this).click();
        }
    });

    // **************************************** search for main items******************************************//

    $(function(){
        typeaheadRecords({
            element: '#id_item',
            url: '/project/search-item/',
            extra: '#id_extra_item'
        })
        window.UpdateTableHeader = tableFixedHeader('.fixed-header');

        $(window).on('resize', function() {
            if (typeof UpdateTableHeader === 'function') {
                UpdateTableHeader.call();
            }
        })
    });

    // **************************************** search for custom items******************************************//

    $(function(){
        typeaheadRecords2({
            element: '#id_custom_item_search',
            url: '/project/search-custom-item/'
        })
    });

    // ************************************** search for custom asseblies items *********************************//

    $(function(){
        typeaheadRecords3({
            element: '#id_search_items',
            url: '/project/search-item/',
            extra: '#id_extra_item_1'
        })
    });

    // ***************************************** end of search functionality ************************************//

    function executeAsync() {
      return new Promise(resolve => {
         intrvl = setInterval(function(){ if(is_all_items_loaded){
            initializeSelectedIndex();
            clearInterval(intrvl)
            resolve(&quot;Done!&quot;)
        }; }, 3000);
      });
    }

    async function asyncCall() {
      const result = await executeAsync();
      // expected output: 'resolved'
    }

    $(document).ready(function(){
        
        moveNextQuantityEventBind();

        if(toggleCost == 'true'){
            toggle_cost();
        }
        $('#id_additem_form').submit(function(e){
            save_input($(this));
        });
        $('#id_cus_additem_form').submit(function(e){
            save_input($(this));
        });
    });

    $(document).on('subconstruction_loaded', function(){
        inputs = localStorage.getItem('inputs');
        if (inputs){
            inputs = JSON.parse(inputs);
        }
        if(inputs &amp;&amp; inputs['id_item_sub_group_no_items'] &amp;&amp; inputs['id_item_sub_group_no_items'] != 'null')
        {
            document.getElementById(&quot;id_item_sub_group_no_items&quot;).value = inputs['id_item_sub_group_no_items']
            $('#id_item_sub_group_no_items').trigger('change');
            $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;)
            $(&quot;.item--suggestion-2&quot;).css(&quot;display&quot;, &quot;none&quot;)
        }
        else{
            $(&quot;#id_item_sub_group_no_items&quot;).val('');
            subgroup = ''
        }
    })
    $(document).on('subconstruction_loaded1', function(){
        inputs = localStorage.getItem('inputs');
        if (inputs){
            inputs = JSON.parse(inputs);
        }
        if(inputs &amp;&amp; inputs['id_item_sub_group_no_items_1'])
        {
            document.getElementById(&quot;id_item_sub_group_no_items_1&quot;).value = inputs['id_item_sub_group_no_items_1']
            $('#id_item_sub_group_no_items_1').trigger('change');
            $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;)
            $(&quot;.item--suggestion-2&quot;).css(&quot;display&quot;, &quot;none&quot;)
        }
        else{
            $(&quot;#id_item_sub_group_no_1&quot;).val('');
            subgroup = ''
        }
    })
    $(document).on('item_subgroup', function(){
        inputs = localStorage.getItem('inputs');
        if (inputs){
            inputs = JSON.parse(inputs);
        }
        if(inputs &amp;&amp; inputs['id_item_sub_group_no'] &amp;&amp; inputs['id_item_sub_group_no'] != 'null')
        {
            $(&quot;#id_item_sub_group_no&quot;).val(inputs['id_item_sub_group_no'])
            $('#id_item_sub_group_no').trigger('change');
            $(&quot;.item--suggestion&quot;).css(&quot;display&quot;, &quot;none&quot;)
            $(&quot;.item--suggestion-2&quot;).css(&quot;display&quot;, &quot;none&quot;)
            subgroup = $(&quot;#id_item_sub_group_no&quot;).val()
        }
        else{
            $(&quot;#id_item_sub_group_no&quot;).val('');
            subgroup = ''
        }
    })

    $('#id_additem_form :input').change(function(e){
        $('#id_additem_form button').removeAttr('disabled');
    });

    $(&quot;.bulk-tools, .pre-dis&quot;).on(&quot;click&quot;, async function(e){
        if(is_all_items_loaded){
            $(&quot;#id_processing&quot;).hide();
        }
        else{
            e.preventDefault();
            e.stopPropagation();
            $(&quot;#id_processing&quot;).show();
            res = await new Promise(function(resolve, reject) {
                setInterval(function(){ if(is_all_items_loaded){resolve(&quot;Done!&quot;)}; }, 3000);
            });
            $(this).click();
        }

    })

    $(&quot;#notificationConfirmModal&quot;).on(&quot;hide.bs.modal&quot;, function(){
        window.location.reload();
    })

    var bulk_tool_temp;

    RowModel = (function(){

        function RowModel(model_json){
            for(key in model_json) {
                this[key] = model_json[key];
            }
        }

        return RowModel;
    })();

    var RowViewModel = (function () {
        function RowViewModel(form, model, formViewModel, rowId, status, validation_rule, change_rule) {
            this.form = form;
            this.model = model;
            this.rowId = rowId;
            this.validation_rule = validation_rule;
            this.change_rule = change_rule;
            this.status = status;

            /*
                {
                    'name': function(value){

                    },
                    ''
                }

                {
                    'name': function(rowId){
                        return {
                            is_valid: is_valid,
                            error_message: error_message
                        }
                    }
                }
            */

            this.formViewModel = formViewModel;

            this.view = new RowView(form, this, this.formViewModel.rowTemplate, status, rowId);
            if (status == 'updated'){

                my_event = $.Event( &quot;change&quot;, { data: { newly_created: true } } )
            }
            else{
                my_event = $.Event( &quot;change&quot;)
            }

            $ths = this;
            $(this.view.rowElement).find(':input').each(function(){$ths.ChangedInView($(this).attr('name'), my_event)});
        }

        RowViewModel.prototype.ChangedInView = function (field_name, e) {
            old_value = this.model[field_name];
            $('.asset_class_setup').multiselect({
                buttonWidth: '100%',
                nonSelectedText: 'Select',
                enableFiltering: true,
                enableCaseInsensitiveFiltering: true,
                maxHeight: 300,
            });
            if (e.data == undefined){
                if (field_name == &quot;asset_class&quot;){
                    var value = $('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').val();
                    var $this = $('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]');
                    var row_id = this.rowId
                    var $this = this

                    // if (value){
                        // $.ajax({
                        //     type: 'GET',
                        //     url: '/project/map-asset-class-cost-setup/',
                        //     data:{
                        //         'asset_id': value,
                        //         'property_id': '',
                        //     },
                        //     success: function(response){
                        //         if (response.recovery_code){
                        //             $('#row-'+row_id+ ' [name=&quot;recovery_period&quot;]').val(response.recovery_code)

                        //             var new_field_name = &quot;recovery_period&quot;
                        //             $this.model[new_field_name] = response.recovery_code
                        //             if($this.change_rule.hasOwnProperty(new_field_name)){
                        //                 new_value = $this.model[new_field_name];
                        //                 $this.change_rule[new_field_name]('#row-'+row_id, old_value, new_value, $this, e);
                        //             }
                        //         }
                        //     }

                        // });
                    // }
                }
            }
            if($('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').is('input[type=&quot;checkbox&quot;]')){
                if($('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').prop('checked')){
                    value = 'on'
                }
                else{
                    value = 'off'
                }
                this.model[field_name] = value;
            }else{
                this.model[field_name] = $('#row-'+this.rowId + ' [name=&quot;'+field_name+'&quot;]').val();
            }

            if(this.change_rule.hasOwnProperty(field_name)){
                new_value = this.model[field_name];
                this.change_rule[field_name]('#row-'+this.rowId, old_value, new_value, this, e);
            }
        };

        RowViewModel.prototype.ChangedInViewModel = function (field_name, field_value, newly_created=false) {

            this.model[field_name] = field_value;

            this.view.ChangedInViewModel(field_name, field_value, newly_created);
        };

        RowViewModel.prototype.validate = function() {
            valid = true;
            row_validation_result = {}
            for(field_name in this.validation_rule){
                validation_result = this.validation_rule[field_name](this.model[field_name], this.model);
                valid = valid &amp;&amp; validation_result[0]
                row_validation_result[field_name] = validation_result;
            }
            this.show_validation_result(row_validation_result);
            return valid;
        };

        RowViewModel.prototype.show_validation_result = function(row_validation_result) {
            var _this = this;
            Object.keys(row_validation_result).forEach(function(value){
                if(!row_validation_result[value][0]){
                    $('#row-'+_this.rowId+&quot; [name='&quot;+value+&quot;']&quot;).parent(&quot;div&quot;).addClass('has-error');
                }
                else{
                    $('#row-'+_this.rowId+&quot; [name='&quot;+value+&quot;']&quot;).parent(&quot;div&quot;).removeClass('has-error');
                }
            })
        }

        RowViewModel.prototype.deleteRow = function () {
            this.formViewModel.deleteRow(this.rowId);
        }

        return RowViewModel;
    })();

    test = {}
    var RowView = (function () {
        function RowView(form, viewModel, template, status, rowId) {
            this.form = form;
            $(form).append(&quot;&lt;div id='row-&quot; +rowId +&quot;' class='row &quot;+ status +&quot;'>&lt;/div>&quot;)
            this.rowElement = document.getElementById('row-'+rowId)
            this.viewModel = viewModel;
            var _this = this;

            for (element_index in template) {
                $(this.rowElement).append(template[element_index].clone());
            }

            for (field_name in this.viewModel.model) {
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').val(this.viewModel.model[field_name]);
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').on(&quot;change keyup keypress keydown&quot;, this.ChangedInView(field_name));
            }

            $(this.rowElement).find('.delete').on(&quot;click&quot;, function () {
                return _this.deleteRow();
            });
        }

        RowView.prototype.ChangedInView = function (field_name) {
            var _this = this;
            return function (e){
                _this.viewModel.ChangedInView(field_name, e);
            }
        };

        RowView.prototype.ChangedInViewModel = function (field_name, field_value, newly_created=false) {
            if($(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').is('input[type=&quot;checkbox&quot;]')){
                if(field_value=='on'){
                    $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').prop('checked', 'checked')
                }
                else{
                    $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').prop('checked', false);
                }
            }
            if(field_name == 'asset_class'){
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').multiselect('select', field_value)
            }
            $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').val(field_value);

            if(field_name != 'subgroup'){
                console.log($(this.rowElement).find('[name=&quot;'+field_name+'&quot;]'))
                console.log($(this.rowElement))
                test[field_name] = this.rowElement
                console.log(field_name)
                my_event = $.Event( &quot;change&quot;, { data: { newly_created: newly_created } } )
                $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').trigger(my_event);
            }else{
                console.log($(this.rowElement).find('[name=&quot;'+field_name+'&quot;]'))
                console.log(field_name)
            }
        };

        RowView.prototype.deleteRow = function () {
            var value = $(this.rowElement).children('[data-name=&quot;total_cost&quot;]').children('input[name=&quot;total_cost&quot;]').val().replace(/,/g, '');
            var total_value = $('#sum_total_cost_setup').val().replace(/,/g, '');
            var new_value = parseFloat(total_value) - parseFloat(value)
            var setup_value = new_value + parseFloat($('#cost_setup_sum').val())
            $('#sum_total_cost_setup').val(new_value.toFixed(2).toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;))
            $('#sum_contract_total').val(setup_value.toFixed(2).toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, &quot;$1,&quot;))
            $(this.rowElement).remove();
            this.viewModel.deleteRow();
        };

        return RowView;
    })();

    var FormView = (function (){
        function FormView(form, viewModel, addButton, saveButton) {
            this.form = form;
            this.viewModel = viewModel;
            var _this = this;

            $(addButton).on(&quot;click&quot;, function () {
                return _this.addRow();
            });

            $(saveButton).on(&quot;click&quot;, function () {
                return _this.save($(this).data('url'));
            });
        }

        FormView.prototype.ChangedInView = function (field_name) {
            return function (field_value){_this.viewModel.ChangedInView(field_name, field_value);}
        };

        FormView.prototype.ChangedInViewModel = function (field_name, field_value) {

            $(this.rowElement).find('[name=&quot;'+field_name+'&quot;]').val(field_value);
        };

        FormView.prototype.addRow = function () {
            this.viewModel.addRow();
        };

        FormView.prototype.save = function (next_url) {
            this.viewModel.save(next_url);
        };

        return FormView;
    })();

    var FormViewModel = (function () {
        function FormViewModel(form, addButton, saveButton, validation_rule, change_rule, previous_object, rowTemplate, modelTemplate, globalStatesTemplate, stateFunction, rowFunction, success_url, sortorder) {
            this.form = form;
            this.model = {};
            this.rowTemplate = rowTemplate;
            this.view = new FormView(form, this, addButton, saveButton);
            this.validation_rule = validation_rule;
            this.change_rule = change_rule;
            this.modelTemplate = modelTemplate;
            this.globalStatesTemplate = globalStatesTemplate;
            this.stateFunction = stateFunction;
            this.rowFunction = rowFunction;
            this.success_url = success_url;
            this.saveButton = saveButton;

            this.globalStates = {}

            for(state in this.globalStatesTemplate){
                this.globalStates[state] = {}
                for(id in this.globalStatesTemplate[state]){
                    this.globalStates[state][id] = {
                        'status': 'existing',
                        'value': this.globalStatesTemplate[state][id]
                    }
                }
            }

            for(state_manager in this.stateFunction){
                this.stateFunction[state_manager].initialize(this);
            }

            for (object in previous_object){
                this.addRow(object, previous_object[object]);
            }

            $.each(sortorder,function(index,value){
                if($.inArray(value, sortorder)!==-1){
                    $(form).append($('#'+value));
                }
            });

            $(form+' div').each(function(){
                if($.inArray($(this).attr('id'), sortorder)==-1){
                    // inArray will return -1, if the element was not found in array
                    $(form).append($('#'+$(this).attr('id')));
                }
            });
        }

        FormViewModel.prototype.addState = function(name, id, label, extra_context){
            var _this =this;
            if(!this.globalStates[name].hasOwnProperty(id)){
                this.globalStates[name][id] = {
                    'status': 'new',
                    'value': ''
                }
                this.globalStates[name][id]['value'] = label;
                this.stateFunction[name].added(_this, id, label, extra_context);
            }
            else{
                this.updateState(name, id, label, extra_context);
            }
        }

        FormViewModel.prototype.updateState = function(name, id, label, extra_context){
            var _this = this;
            this.globalStates[name][id]['value'] = label;
            this.stateFunction[name].updated(_this, id, label, extra_context);
        }

        FormViewModel.prototype.removeState = function(name, id, extra_context){
            var _this = this;
            delete this.globalStates[name][id];
            this.stateFunction[name].removed(_this, id, extra_context);
        }

        FormViewModel.prototype.addRow = function (rowId, data_model){
            $this = this
            var model_template_value = JSON.parse(JSON.stringify(this.modelTemplate))
            var model_list = Object.keys(this.model).filter(function(x){ return $this.model[x].status != 'deleted' })
            if (model_list.length >= 1){
                var last_row_id = model_list[model_list.length - 1]
                model_template_value['cus_unit'] = this.model[last_row_id]['model']['cus_unit']
                model_template_value['quantity'] = this.model[last_row_id]['model']['quantity']
                model_template_value['cost_source'] = this.model[last_row_id]['model']['cost_source']
                model_template_value['item_group_no'] = this.model[last_row_id]['model']['item_group_no']
                model_template_value['recovery_period'] = this.model[last_row_id]['model']['recovery_period']
                model_template_value['location'] = this.model[last_row_id]['model']['location']
                model_template_value['indirect_cost_source'] = this.model[last_row_id]['model']['indirect_cost_source']
                model_template_value['asset_class'] = this.model[last_row_id]['model']['asset_class']
            }

            var model = new RowModel(model_template_value)

            if(!rowId){
                uuid = uuidv4();
                status = 'created';
            }
            else{
                uuid = rowId;
                status = 'updated';
            }
            var new_row = new RowViewModel(this.form, model, this, uuid, status, this.validation_rule, this.change_rule);
            this.model[uuid] = new_row;

            if(data_model){
                for(field_name in data_model){
                    this.model[uuid].ChangedInViewModel(field_name, data_model[field_name], newly_created=true)
                }
            }

            if(status == 'updated'){
                this.rowFunction.rowAdded(this, rowId)
            }
        }

        FormViewModel.prototype.save = function(next_url) {
            is_valid = true;
            for(row in this.model){
                if(this.model[row].status != 'deleted'){
                    row_valid = this.model[row].validate();
                    is_valid = is_valid &amp;&amp; row_valid;
                }
            }
            var _this = this;

            if(is_valid){
                $(_this.saveButton).html(
                    '&lt;img src=&quot;/static/images/spin.svg&quot; style=&quot;height: 25px; margin-top: -5px;&quot;>')
                $(_this.saveButton).attr(&quot;disabled&quot;, true);

                body = {
                    'created': {},
                    'deleted': [],
                    'updated': {},
                    'global_states':{}
                }
                for(row in this.model){

                    if(this.model[row].status == 'created'){
                        body['created'][this.model[row].rowId] = this.model[row].model;
                    }
                    else if(this.model[row].status == 'deleted'){
                        body['deleted'].push(this.model[row].rowId);
                    }
                    else{
                        body['updated'][this.model[row].rowId] = this.model[row].model;
                    }
                }

                body['global_states'] = this.globalStates;
                body[&quot;order&quot;] = $(this.form).sortable(&quot;toArray&quot;);

                $.ajax({
                    url: window.location.href,
                    type: 'post',
                    success: function (data) {
                        window.location.href = next_url;
                    },
                    data: {
                        'body': JSON.stringify(body),
                        'csrfmiddlewaretoken': &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;
                    }
                });
            }
        }

        FormViewModel.prototype.deleteRow = function (rowId){
            this.rowFunction.rowDeleted(this, rowId)
            if(this.model[rowId].status == 'created'){
                delete this.model[rowId];
            }
            else if (this.model[rowId].status == 'updated'){
                this.model[rowId].status = 'deleted';
            }
        }

        return FormViewModel;
    })();

    jQuery.fn.selectText = function(){
       var doc = document;
       var element = this[0];
       if (doc.body.createTextRange) {
           var range = document.body.createTextRange();
           range.moveToElementText(element);
           range.select();
       } else if (window.getSelection) {
           var selection = window.getSelection();
           var range = document.createRange();
           range.selectNodeContents(element);
           selection.removeAllRanges();
           selection.addRange(range);
       }
    };

    $(document).on(&quot;click&quot;, &quot;.analysis_window_tab&quot;, function(){
        if(!$(this).hasClass(&quot;active&quot;)){
            property_id = $(this).data(&quot;property&quot;);
            tab = $(this).data(&quot;tab&quot;);
            estimate_result(property_id, tab=tab);
        }
    })

    var NotificationManager = (function () {
        function NotificationManager() {
            this.notification_container = &quot;.notification-container&quot;;
            this.is_container_open = false;
            this.limit = 4;
            this.offset = 0;
            this.has_more = true;
            this.loading = false;
            this.url_to_new_content = &quot;/get-notifications/&quot;;
            this.last_notification_id = null;
            this.new_notification = false;
            this.unread_count = null;
            var $this = this;
        }

        NotificationManager.prototype.get_notifications = function(popup=false){
            var $this = this;
            if($this.has_more){
                data = {
                    'last_notification_id': $this.last_notification_id,
                    'limit': $this.limit,
                    'offset': $this.offset,
                    'csrfmiddlewaretoken': &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;,
                    'popup': popup
                }
                $.ajax({
                    type:'POST',
                    url: $this.url_to_new_content,
                    data: data,
                    success:function(data){
                        $this.loading = false;
                        $this.has_more = data.has_more;
                        $this.offset = $this.offset + $this.limit;
                        $this.last_notification_id = data.last_notification_id;
                        $($this.notification_container).append(data.content);
                        $(this.notification_container).scroll()
                    }
                });
            }
        }

        NotificationManager.prototype.update_notification = function(data){
            // $(&quot;#id-num&quot;).html(data.count)
            $(&quot;.id-num&quot;).html(data.count)
            if(this.is_container_open){
                this.show_refresh_option();
            }else{
                $(&quot;#id-comment&quot;).show();
                $(&quot;.id-num&quot;).show();
            }
        }

        NotificationManager.prototype.reset_notification = function(){
            this.last_notification_id = null;
            this.offset = 0;
            this.has_more = true;
            $(this.notification_container).html(&quot;&quot;);
        }

        return NotificationManager;
    })();

    var notification_manager;

     $(document).ready(function(){
        notification_manager = new NotificationManager();
        
        setupWebSocket()
        

        $('.import_cost_setup').on('click', function(e){
            var id = $(this).data('id');
            var type = $(this).data('type');

            $('#upload_cost_setup').attr('onclick', 'uploadCostFile(&quot;&quot;, &quot;'+type+'&quot;)');

            $('#importCostSetup').modal('show');


        });
    })

    var wsUri = &quot;wss://dev.segstream.com/ws/?user_id=26&quot;;
    function setupWebSocket() {
        websocket = new WebSocket(wsUri) ;
        websocket.onopen = function(evt) { onopen(evt) } ;
        websocket.onmessage = function(evt) { onMessage (evt)};
        websocket.onclose = function(e) {
            console.log('Socket is closed. Reconnect will be attempted in 1 second.', e.reason);
            setTimeout(function() {
              setupWebSocket();
            }, 1000);
          };
    };

    function onopen (evt) {

        // websocket.send('{&quot;type&quot;: &quot;ping&quot;}')
    }

    function popupNotificationName(name, next_url){
        $('#notification_property_name').html(name);
        $('#next_link_worksheet').attr('href', next_url);
        $('#notificationConfirmModal').modal('show');

        setTimeout(function(){
            localStorage.setItem(&quot;notification_modal&quot;, false);
        }, 5000);
    }

    function onMessage (evt) {
        data = JSON.parse(evt.data);
        if(data.type == 'notification'){

            
            var notification_property_id = null;
            

            if(!data.property_id || (data.property_id != notification_property_id)){
                if(data.notification_type != 'export'){
                    localStorage.setItem(&quot;notification_modal&quot;, true);

                    localStorage.setItem(&quot;property_name&quot;, data.property_name);
                    localStorage.setItem(&quot;next_url&quot;, data.next_url);
                    // profile_manager.open_up_notification_popup(data);
                    popupNotificationName(data.property_name, data.next_url)
                }
            }


        }
        if(data.type == 'export'){
            if (window.location.href == data.location){
                // window.location.href = data.data;

                var url = data.data;
                var sheet_name = ''

                var filename = data.filename;

                // var request = new XMLHttpRequest();
                // request.open('GET', url, true);
                // request.responseType = 'blob';
                // request.onload = function() {
                //    var link = document.createElement('a');
                //    document.body.appendChild(link);
                //    link.href = window.URL.createObjectURL(request.response);
                //    link.download = filename;
                //    link.click();
                // };
                // request.send();

                var link = document.createElement('a');
               document.body.appendChild(link);
               link.href = url;
               link.download = filename;
               link.click();
                $(&quot;#id_processing&quot;).hide();

                $.ajax({
                    'type': &quot;GET&quot;,
                    &quot;url&quot;: &quot;/read/report/&quot;+data.report_id
                })
            }
            else{
                    profile_manager.open_up_notification_popup({'data': {
                    'count': &quot;!&quot;
                }});
            }
        }

        if(data.type == 'Algorithm Verify'){
            $(&quot;#verify_result&quot;).html(data.message);
            $(&quot;#verify_result&quot;).show();
            $(&quot;.statement&quot;).each(function(){
                n_width = $(this).children(&quot;.statement_name&quot;).css(&quot;width&quot;);
                v_width = $(this).children(&quot;.statement_value&quot;).css(&quot;width&quot;);
                n_width = parseFloat(n_width.slice(0, n_width.length-2))
                v_width = parseFloat(v_width.slice(0, v_width.length-2))
                if(n_width > v_width){
                    $(this).children(&quot;.statement_name, .statement_value&quot;).css(&quot;width&quot;, n_width)
                }else{
                    $(this).children(&quot;.statement_name, .statement_value&quot;).css(&quot;width&quot;, v_width)
                }
                $(this).children(&quot;.statement_name&quot;).css(&quot;display&quot;, &quot;inline-block&quot;)
                $(this).children(&quot;.statement_value&quot;).css(&quot;display&quot;, &quot;none&quot;)
            })
        }

        if(data.type == &quot;Algorithm Component Extracted&quot;){
            $(&quot;#component-ready-&quot;+data.id).text(data.count)
        }

        if(data.type == &quot;Enable Refresh Algorithm&quot;){
            $('#refresh_algorithm').prop('disabled', false);
        }

        if(data.type == &quot;Algorithm Item Completed&quot;){

            $.ajax({
                type: 'POST',
                url: '/algorithm-component/',
                data: {
                    'item_id': data.id,
                    'property_id': data.property_id,
                    'csrfmiddlewaretoken': &quot;Dg0oWadbsp3iAkmsRsJ282SDm9NhCvaoYATmm1YOYo9cB4d3aZMD4RsyImvcXxj2&quot;
                },
                success: function(response){
                    $(&quot;#component-not-ready-row-&quot;+response.id).replaceWith(response.components)
                }
            });
        }

        if(data.type == &quot;enhanced_property_algorithm_completed&quot;){
            $(&quot;#&quot;+data.algorithm_id+&quot;-processing-icon&quot;).hide()
            $(&quot;#&quot;+data.algorithm_id+&quot;-saved-icon&quot;).show()

        }

        if(data.type == &quot;enhanced_property_algorithm_entered&quot;){
            $(&quot;#&quot;+data.algorithm_id+&quot;-processing-icon&quot;).show()
        }

        if(data.type == &quot;building_processed&quot;){
            if(data.all_building_submit){
                building_ui.move_to_sheet()
            }
            building_ui.saved(data.building_id);
        }
    }

    var AlgorithmIntervalMap = {}
    var animateHeader = (function() {
        function constructor(algorithm_id){
            this.algorithm_id = algorithm_id;

            this.interval = setInterval(function(){$(&quot;.algorithm-list #algorithm-&quot;+algorithm_id).animate({
                opacity: $(&quot;.algorithm-list #algorithm-&quot;+algorithm_id).css('opacity') == 0.5 ? 0.8 : 0.5
            }, 250);}, 250)
        }
        return constructor;
    })();

    var user_clicked_notification_popup = false;
    function checkInteractionWithNotificationPopup() {
      return new Promise(resolve => {
        count = 0;
        $(&quot;.myBar, .myProgress&quot;).show();
        xinterval = setInterval(function(){
            var progressBarWidth = (10000-count) * $(&quot;.myProgress&quot;).width() / 10000;
            $(&quot;.myBar&quot;).css({&quot;width&quot;: progressBarWidth})
            if(count >= 10000){
                if(!user_clicked_notification_popup){

                close_notification_popup_and_show_count();
                $(&quot;#dropdown_menu_container&quot;).removeClass(&quot;notification_popup&quot;)
        $(&quot;.myBar, .myProgress&quot;).hide();
                profile_manager.status = &quot;closed&quot;;
            }
                clearInterval(xinterval);
                user_clicked_notification_popup = false
                resolve(&quot;Done!&quot;)
            }
            else{
                if(!isHovering(&quot;#dropdown_menu_container&quot;)){
                    count += 250
                }else{
                    count +=0;
                }
            }
        }, 250)
      });
    }

    async function hideNotif() {
      const result = await checkInteractionWithNotificationPopup();
      // expected output: 'resolved'
    }

    $(document).ready(function(){
        $('#notification-menu').on('shown', function () {
            notification_manager.is_container_open = true;
            notification_manager.reset_notification();
            notification_manager.get_notifications();
            $(&quot;#id-comment&quot;).hide();
            $(&quot;#id-num&quot;).hide();
        });

        $('#notification-menu').on('shown_notif', function () {
            notification_manager.is_container_open = true;
            notification_manager.reset_notification();
            notification_manager.get_notifications();
        });


        $('#notification-menu').on('hidden', function () {
            notification_manager.is_container_open = false;
        });

        notification_modal = localStorage.getItem(&quot;notification_modal&quot;);

        if (notification_modal == &quot;true&quot;){
            var name = localStorage.getItem(&quot;property_name&quot;);
            var next_url = localStorage.getItem(&quot;next_url&quot;);
            popupNotificationName(name, next_url);

        }
    });

    $('#navbarDropdown').on('click', function (event) {
        $(this).parent().toggleClass('open');
        if($(this).parent().hasClass(&quot;open&quot;)){
            $('#notification-menu').trigger(&quot;shown&quot;);
        }
        else{
            $('#notification-menu').trigger(&quot;hidden&quot;);
        }
    });

    var ProfileManager = (function () {
        function ProfileManager() {
            this.status = &quot;closed&quot; // Possible options: root, notification, management, notification_popup
            this.notification_count = null;
            this.home_div = &quot;#home-options&quot;;
            this.management_div = &quot;#management-options&quot;;
            this.dropdown_menu_container = &quot;#dropdown_menu_container&quot;
            this.notification_div = &quot;#notification-menu&quot;;
        }

        ProfileManager.prototype.open_management = function(){
            profile_manager.status = 'management';
            $(profile_manager.dropdown_menu_container).html($(profile_manager.management_div).html())
        }

        ProfileManager.prototype.open_notification = function(){
            profile_manager.status = 'notification';
            $(&quot;.id-num, .id-comment&quot;).hide();
            notification_manager.reset_notification();
            notification_manager.get_notifications();
            $(profile_manager.dropdown_menu_container).html($(profile_manager.notification_div).html())
        }

        ProfileManager.prototype.go_home = function(){
            profile_manager.status = 'root';
            $(profile_manager.dropdown_menu_container).html($(profile_manager.home_div).html())
        }

        ProfileManager.prototype.open_up_notification_popup = function(data){
            notification_manager.update_notification(data.data)
            this.status = &quot;notification_popup&quot;;
            $('#dropdown_link').click();
            newNotification = true;
            userInteractedWithNotificationPopup = false;
            hideNotif();
        }

        return ProfileManager;
    })();

    var profile_manager = new ProfileManager();

    $('#nav-drpdwn2').on('show.bs.dropdown', function(){
        if(profile_manager.status == &quot;notification_popup&quot;){
            $(profile_manager.dropdown_menu_container).addClass(&quot;notification_popup&quot;);
            notification_manager.reset_notification();
            notification_manager.get_notifications(popup=true);
            $(profile_manager.dropdown_menu_container).html($(profile_manager.notification_div).html())
        }
        else{
            profile_manager.status = 'root';
            $(profile_manager.dropdown_menu_container).html($(profile_manager.home_div).html())
        }
    });

    $(document).on(&quot;click&quot;, &quot;.management-options-link&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        profile_manager.open_management();
    })

    $(document).on(&quot;click&quot;, &quot;.notification-panel-link&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        profile_manager.open_notification();
    })

    $(document).on(&quot;click&quot;, &quot;.go-back-to-home&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        profile_manager.go_home();
    })

    $(document).on(&quot;click&quot;, &quot;.notification_popup&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        $(this).removeClass(&quot;notification_popup&quot;);
        $(&quot;.myBar, .myProgress&quot;).hide();
        profile_manager.open_notification();
        user_clicked_notification_popup = true;
    })

    $('#nav-drpdwn2').on(&quot;hide.bs.dropdown&quot;, function(e){
        if($(&quot;#dropdown_menu_container&quot;).hasClass(&quot;notification_popup&quot;)){
            e.preventDefault();
            e.stopPropagation();
        }
    })

    $(function(){

        $('*').hover(function(){
            $(this).data('hover',1); //store in that element that the mouse is over it
        },
        function(){
            $(this).data('hover',0); //store in that element that the mouse is no longer over it
        });


        window.isHovering = function (selector) {
            return $(selector).data('hover')?true:false; //check element for hover property
        }
    });

    $(window).on(&quot;unload&quot; ,function() {
        if(profile_manager.status == &quot;notification_popup&quot;){
            window.localStorage.setItem(&quot;notification_popup&quot;, &quot;true&quot;);
        }
    })

    $(document).ready(function(){
        if(localStorage.getItem(&quot;notification_popup&quot;) == &quot;true&quot;){
            
            var show_notif = true;
            

            if(show_notif){
                profile_manager.open_up_notification_popup({'data': {
                    'count': &quot;0&quot;
                }})
            }
            localStorage.setItem(&quot;notification_popup&quot;, &quot;false&quot;)
        }
    })

    function checkValue(value){
        if ($(value).val()){
            $(&quot;.search-form .form-group&quot;).addClass('hover');
        }else{
            $(&quot;.search-form .form-group&quot;).removeClass('hover');
            $('span.form-control-remove').removeClass('search-remove-active').addClass('search-remove-hide');
        }
    }

    function inputValue(){
        $(&quot;.search-form .form-group&quot;).addClass('hover');
    }

    function close_notification_popup_and_show_count(){
        $('#nav-drpdwn2').removeClass('open');
        $('#notification-menu').trigger(&quot;hidden&quot;)
        $(&quot;.id-num&quot;).show();
        $(&quot;id-comment&quot;).show();
    }

    $(document).on(&quot;click&quot;, &quot;.close-notification-popup&quot;, function(e){
        e.preventDefault();
        e.stopPropagation();
        close_notification_popup_and_show_count();
    })

    var global_check = true

    function globalCheck(check){
        global_check = check
    }

    $(document).on(&quot;click&quot;, &quot;.prev_items_with_assemblies&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');

        if($(this).data('check_url') == 'custom'){
            $('#cus_button_clicked').val(check_next);
            update_cus_item_details($('#id_cus_additem_form'), async=false);
        }else{
            update_item_details($('#id_update_item_form'));
        }

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index &lt; 1){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        if(global_check){
            $(&quot;tr > td[data-id]&quot;).eq(index-1).dblclick();
        }
        $('#cus_button_clicked').val('');
    })

    $(document).on(&quot;click&quot;, &quot;.prev_items_with_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index &lt; 1){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr > td[data-id]&quot;).eq(index-1).dblclick();
        $('#button_clicked').val('');

    })


    $(document).on(&quot;click&quot;, &quot;.next_items_with_assemblies&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');

        if($(this).data('check_url') == 'custom'){
            $('#cus_button_clicked').val(check_next);
            update_cus_item_details($('#id_cus_additem_form'), async=false);
        }else{
            update_item_details($('#id_update_item_form'));
        }

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index >= $(&quot;tr > td[data-id]&quot;).length ){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        if(global_check){
            $(&quot;tr > td[data-id]&quot;).eq(index+1).dblclick();
        }
        $('#cus_button_clicked').val('');
    })

    $(document).on(&quot;click&quot;, &quot;.next_items_with_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index >= $(&quot;tr > td[data-id]&quot;).length ){
            e.preventDefault();
            e.stopPropagation();
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr > td[data-id]&quot;).eq(index+1).dblclick();
        $('#button_clicked').val('');
    })

    $(document).on(&quot;click&quot;, &quot;.prev_items_without_assemblies&quot;, function(e){
        e.preventDefault();
        if($(this).data('check_url') == 'assembly'){
            update_item_details($('#id_update_item_form'));
        }

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = (&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2-1).dblclick();
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2-1).dblclick();
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index-1).dblclick();
        return
    })

    $(document).on(&quot;click&quot;, &quot;.prev_items_without_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = (&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2-1).dblclick();
                $('#button_clicked').val('');
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2-1).dblclick();
                $('#button_clicked').val('');
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index-1).dblclick();
        $('#button_clicked').val('');
        return
    })

    $(document).on(&quot;click&quot;, &quot;.next_items_without_assemblies_items&quot;, function(e){
        e.preventDefault();
        var check_next = $(this).data('check_next');
        $('#button_clicked').val(check_next);
        update_assemb_item_details($('#id_cus_additem_form_items'));

        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2+1).dblclick();
                $('#button_clicked').val('');
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2+1).dblclick();
                $('#button_clicked').val('');
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index+1).dblclick();
        $('#button_clicked').val('');
        return
    })

    $(document).on(&quot;click&quot;, &quot;.next_items_without_assemblies&quot;, function(e){
        e.preventDefault();
        if($(this).data('check_url') == 'assembly'){
            update_item_details($('#id_update_item_form'));
        }
        item_id = $(this).data(&quot;item-id&quot;);
        index = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index($(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+item_id+&quot;']&quot;));
        if(index == -1){
            if (!$(this).data(&quot;parent-id&quot;)){
                index2 = $(&quot;tr > td[data-id]&quot;).index($(&quot;tr > td[data-id='&quot;+item_id+&quot;']&quot;));
                if (index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr > td[data-id]&quot;).eq(index2+1).dblclick();
                return
            }
            else{
                prnt = $(&quot;td[data-id='&quot;+$(this).data(&quot;parent-id&quot;)+&quot;']&quot;);
                index2 = $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).index(&quot;tr:not(.id_assemb_items) > td[data-id='&quot;+prnt+&quot;']&quot;);
                if(index2 == -1){
                    e.preventDefault();
                    e.stopPropagation();
                }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
                $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index2+1).dblclick();
                return
            }
        }
        // $(&quot;#item_details&quot;).modal(&quot;hide&quot;);
        $(&quot;tr:not(.id_assemb_items) > td[data-id]&quot;).eq(index+1).dblclick();
        return
    })








  @keyframes intercom-lightweight-app-launcher {
    from {
      opacity: 0;
      transform: scale(0.5);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  @keyframes intercom-lightweight-app-gradient {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes intercom-lightweight-app-messenger {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .intercom-lightweight-app {
    position: fixed;
    z-index: 2147483001;
    width: 0;
    height: 0;
    font-family: intercom-font, &quot;Helvetica Neue&quot;, &quot;Apple Color Emoji&quot;, Helvetica, Arial, sans-serif;
  }

  .intercom-lightweight-app-gradient {
    position: fixed;
    z-index: 2147483002;
    width: 500px;
    height: 500px;
    bottom: 0;
    right: 0;
    pointer-events: none;
    background: radial-gradient(
      ellipse at bottom right,
      rgba(29, 39, 54, 0.16) 0%,
      rgba(29, 39, 54, 0) 72%);
    animation: intercom-lightweight-app-gradient 200ms ease-out;
  }

  .intercom-lightweight-app-launcher {
    position: fixed;
    z-index: 2147483003;
    bottom: 20px;
    right: 100px;
    width: 60px;
    height: 60px;
    border-radius: 50%;
    background: #006eb3;
    cursor: pointer;
    box-shadow: 0 1px 6px 0 rgba(0, 0, 0, 0.06), 0 2px 32px 0 rgba(0, 0, 0, 0.16);
    animation: intercom-lightweight-app-launcher 250ms ease;
  }

  .intercom-lightweight-app-launcher:focus {
    outline: none;
    
  }

  .intercom-lightweight-app-launcher-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    position: absolute;
    top: 0;
    left: 0;
    width: 60px;
    height: 60px;
    transition: transform 100ms linear, opacity 80ms linear;
  }

  .intercom-lightweight-app-launcher-icon-open {
    
        opacity: 1;
        transform: rotate(0deg) scale(1);
      
  }

  .intercom-lightweight-app-launcher-icon-open svg {
    width: 28px;
    height: 32px;
  }

  .intercom-lightweight-app-launcher-icon-open svg path {
    fill: rgb(255, 255, 255);
  }

  .intercom-lightweight-app-launcher-icon-self-serve {
    
        opacity: 1;
        transform: rotate(0deg) scale(1);
      
  }

  .intercom-lightweight-app-launcher-icon-self-serve svg {
    height: 56px;
  }

  .intercom-lightweight-app-launcher-icon-self-serve svg path {
    fill: rgb(255, 255, 255);
  }

  .intercom-lightweight-app-launcher-custom-icon-open {
    max-height: 36px;
    max-width: 36px;
    
        opacity: 1;
        transform: rotate(0deg) scale(1);
      
  }

  .intercom-lightweight-app-launcher-icon-minimize {
    
        opacity: 0;
        transform: rotate(-60deg) scale(0);
      
  }

  .intercom-lightweight-app-launcher-icon-minimize svg {
    width: 16px;
  }

  .intercom-lightweight-app-launcher-icon-minimize svg path {
    fill: rgb(255, 255, 255);
  }

  .intercom-lightweight-app-messenger {
    position: fixed;
    z-index: 2147483003;
    overflow: hidden;
    background-color: white;
    animation: intercom-lightweight-app-messenger 250ms ease-out;
    
        width: 376px;
        height: calc(100% - 120px);
        max-height: 704px;
        min-height: 250px;
        right: 100px;
        bottom: 100px;
        box-shadow: 0 5px 40px rgba(0,0,0,0.16);
        border-radius: 8px;
      
  }

  .intercom-lightweight-app-messenger-header {
    height: 75px;
    background: linear-gradient(
      135deg,
      rgb(0, 46, 114) 0%,
      rgb(0, 5, 12) 100%
    );
  }

  @media print {
    .intercom-lightweight-app {
      display: none;
    }
  }
/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
</WebElementEntity>
